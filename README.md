Rust Performance Measurement
====================

## The Const-Static-Value Performance Measurement

To compare the performance between **const** variables and **static** variables for scalar types and arrays.

```bash
cargo bench --bench const_static_value
```

The result indicates that, with compiler optimization, const and static variables have (almost) the same performance.

## The For-Iterator Performance Measurement

To compare the performance among **for loops**, **for-iterator loops** and **iterators** for arrays in stacks or heaps and their slices.

```bash
cargo bench --bench for_iterator
```

The performance ranks are,

1. for_loop_stack, iterator_stack, for_iterator_loop_stack
1. iterator_stack_slice, for_iterator_loop_stack_slice, iterator_heap, iterator_heap_slice, for_iterator_loop_heap, for_iterator_loop_heap_slice
1. for_loop_stack_slice, for_loop_heap, for_loop_heap_slice

And so what? The result indicates that,

1. If your array is in stacks, you should use its owner variable as much as possible.
1. If your array is in heaps, you should avoid using **for loops** on it.
1. If your array is actually a slice, you should avoid using **for loops** on it.

## The Iterator-Enumerate Performance Measurement

To compare the performance between **for loops** and **Enumerate iterators**.

```bash
cargo bench --bench iter_enumerate
```

The performance is about the same.

## The Iterator-Skip Performance Measurement

To compare the performance between **ranged slices** and **Skip iterators**.

```bash
cargo bench --bench iter_skip
```

**Ranged slices** are much better than **Skip iterators**.

## The Iterator-Take Performance Measurement

To compare the performance between **ranged slices** and **Take iterators**.

```bash
cargo bench --bench iter_take
```

The performance is about the same.

## The One-Char-Push Performance Measurement

To compare the performance between `string.push_str("c")` and `string.push('c')`.

```bash
cargo bench --bench one_char_push
```

The result shows that `string.push('c')` is better than `string.push_str("c")` when the char is 1 byte in UTF-8.

## The One-Char-Write Performance Measurement

To compare the performance between `formatter.write_str("c")` and `formatter.write_char('c')`.

```bash
cargo bench --bench one_char_write
```

The result shows that `formatter.write_char('c')` is better than `formatter.write_str("c")` when the char is 1 byte in UTF-8.

## The Push-Str-Write-Fmt Performance Measurement

To compare the performance between `string.push_str(&format!(...))` and `string.write_fmt(format_args!(...)).unwrap()`.

```bash
cargo bench --bench push_str_write_fmt
```

The result shows that `string.write_fmt(format_args!(...)).unwrap()` is better than `string.push_str(&format!(...))`.

## The Vec-Box-Array Performance Measurement

To compare the performance among `[u8; size]` (no_box), `Vec<u8>` (with_vec), `Box<[u8; size]>` (with_box) and `Box<[u8]>` (with_vec_to_box).

```bash
cargo bench --bench vec_box_array
```

The performance ranks of allocation are,

1. alloc_no_box
1. alloc_with_box, alloc_with_vec, alloc_with_vec_to_box
1. alloc_with_box_2

The performance ranks of storing and retrieving are,

1. no_box
1. with_box, with_box_2, with_vec, with_vec_to_box

The **with_box_2** is also the `Box<[u8; size]>` type, but its instance is created by a way which is something like `Box::new([u8; size])`. Even though `Box` aims to allocate memory space in heaps, the statement `Box::new([u8; size])` still needs **size**-bytes space in stacks to initialize the `[u8; size]` instance, and after that, `Box` creates another space in heaps to store the initialized `[u8; size]`. This way to create a `Box<[u8; size]>` instance is slow and still can cause a stack overflow.

Overall, if you need a buffer, the `[u8; size]` type is the best option. If you need an array in heaps, there is no need to make it fixed-size.


## License

[MIT](LICENSE)