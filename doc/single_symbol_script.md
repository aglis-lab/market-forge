# Video Script: "We Built a Matching Engine — Insert Hits 30M Per Second, But Cancel Was a Problem"

---

## HOOK (0:00–0:12)

"So we built a matching engine in Rust. For pure insert, we hit 30 million orders per second. That number looked great. But when we tried to cancel orders at scale, the speed dropped badly. Here is what happened and how we fixed it."

---

## THE PROBLEM (0:12–0:35)

"Every matching engine has the same basic parts. A list of buy orders, a list of sell orders. When prices match, they trade. Simple.

The hard part is doing all three operations fast — insert, match, and cancel.

Our first version used a VecDeque at each price level. Add to the back, take from the front. That is O(1) both ways and very fast. Insert tests went up to 80 million orders with no problem.

But cancel is different. Cancel needs to find one order somewhere in the middle of that list and remove it. That means scanning from the start until you find it, then shifting everything after it to fill the gap. The more orders you have, the slower this gets."

---

## slow_cancel — THE VECDEQUE BRANCH (0:35–1:20)

"This is the slow_cancel branch. Each price level holds an OrderQueue, which is a VecDeque of order metadata. There is actually a TODO comment in the code that says this clearly:

'We can improve this by using prev and next indexes inside the order allocator. But for now, we use a simple search.'

That TODO became our next branch. Here is what the slow path looks like:

```rust
// slow_cancel: order_queue.rs
pub fn find_order(&self, order_id: OrderId) -> Option<(usize, OrderMeta)> {
    self.queue.iter().enumerate()
        .find(|(_, meta)| meta.order_id() == order_id)
        .map(|(index, meta)| (index, *meta))
}

pub fn remove_index(&mut self, index: usize) -> Option<OrderMeta> {
    Some(self.queue.remove(index).unwrap())
}
```

For insert only, this branch goes up to 80 million orders in our tests. VecDeque is that fast when you only add to the back.

But the cancel test stops at 1 million orders. Not because we chose that number. It stops there because above 1 million, the slowdown is so large that the test result has no meaning. And the combined test — insert plus cancel plus replace together — only goes up to 1.5 million. Cancel brings down the whole system."

---

## folder-style — THE ORDERNODE BRANCH (1:20–2:10)

"The folder-style branch solves this with a linked list built directly into the allocator. Each order is stored inside an OrderNode. That node holds the order data, plus a link to the order before it and the order after it in the same memory pool.

```rust
// folder-style: order_allocator.rs
pub struct OrderNode<T> {
    order: T,
    prev_idx: Option<AllocatorIndex>,
    next_idx: Option<AllocatorIndex>,
}
```

Each price level is now just four numbers: the first order index, the last order index, the count, and the total quantity. No VecDeque at all.

The allocator also holds a hash map from order ID to allocator index. So finding any order by ID takes one lookup.

Cancel now works like this:

```rust
// folder-style: cancel.rs
let (order_allocator_idx, order_node) =
    self.order_allocator.try_remove_by_order_id(order_id)?;

if let Some(prev) = order_node.prev_idx().and_then(|i| self.order_allocator.get_mut(i)) {
    prev.set_next_idx(order_node.next_idx());
}
if let Some(next) = order_node.next_idx().and_then(|i| self.order_allocator.get_mut(i)) {
    next.set_prev_idx(order_node.prev_idx());
}
```

One hash map lookup. Update two links. Update the price level. Done. The time is the same whether there are 10 orders at that price or 10 thousand.

The cancel API also got cleaner. In slow_cancel, you have to pass the price and the side just so the engine can find the order. In folder-style, you only pass the order ID. The hash map does the rest."

---

## WHAT ELSE CHANGED (2:10–2:30)

"Two other things also changed.

The index type in the allocator went from u64 to NonZeroU32. That is 8 bytes down to 4. And because it is NonZero, an Option around it also fits in 4 bytes with no extra space needed. The zero value means None. With 30 million orders in memory at the same time, this smaller size helps the CPU cache work better.

The test data also got more realistic. slow_cancel uses simple fixed-price buy and sell pairs. folder-style adds a function called make_realistic_orders. It moves the price up and down using small random steps, puts 70 percent of orders near the top of the book, and gives order sizes a power-law shape where most are small and a few are large. That is how real markets work. You want to test on data that looks like production, not a clean pattern that always goes well."

---

## THE NUMBERS (2:30–2:50)

"Here is the full picture.

slow_cancel insert: tests go up to 80 million orders, speed reaches around 30 million orders per second. Very fast. But add cancel to the mix and the combined test falls to 1.5 million. Cancel breaks it.

folder-style combined workload: tests run from 5 million up to 30 million orders, and the speed stays flat all the way through. The README says 10 to 14 million operations per second on combined work — insert, replace, and cancel together at real-world ratios.

The 30 million limit in the folder-style tests is not a speed limit. It is a memory limit. Above 40 to 50 million live orders, the process runs out of RAM. The speed does not drop — we just ran out of machine. With more RAM, you keep going at the same speed."

---

## WHAT STAYED THE SAME (2:50–3:05)

"The matching logic did not change much. Both branches use a sorted map for price levels — asks go low to high, bids go high to low. Both use a slab for order storage. Both support Limit, Market, IOC, and FOK orders. The Order trait, OrderSpec, and the rkyv data format are the same. folder-style only changed the parts that were slow."

---

## CALL TO ACTION (3:05–3:15)

"It is open source. If you need fast order matching with fast cancel, look at the folder-style branch. If you want to see how we got there and why VecDeque stops working at scale, start with slow_cancel and read the diff. The repo is at github.com/aglis-lab/market-forge."

---

**Repo:** github.com/aglis-lab/market-forge
**Branches:** `slow_cancel` (VecDeque, fast insert, slow cancel) · `folder-style` (OrderNode linked list, O(1) cancel, 10–14M combined ops/sec)

---

*Tone: developer sharing real work. Simple words, short sentences, no filler.*
