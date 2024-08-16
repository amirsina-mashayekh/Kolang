# Kolang
Kolang is a general-purpose programming language. I'm working on this project as a hobby.

## What does *Kolang* mean?
Kolang (Persian: کلنگ; /kolang/) is a Persian word that means *Pickaxe*.

![Minecraft iron pickaxe](Iron_Pickaxe.png)

I chose this name for fun. 😁

The name also contains the part *lang* which can be an abbreviation for *language*; so you can also think of Kolang as a **Kool language**! 😎

## Projects that Inspired Kolang

Here are some projects that have inspired me in the development of Kolang:

- [**TSLang**](https://github.com/amirsina-mashayekh/TSLang-Compiler): A .Net based parser I wrote for [Prof. Ali Gholami Rudi](https://www.rudi.ir/)'s 'Principles of Compiler Design' lecture project

- [**Limoo-Lang**](https://github.com/always-maap/Limoo-Lang): A compiler written for the same lecture by [Mohammad ali Ali panah](https://github.com/always-maap)

- [**Rust**](https://www.rust-lang.org/): This project is fully written in Rust. Also, the syntax of Kolang is much similar to the syntax of Rust.

## Example code
``` Rust
fn add(a: int, b: int): int {
    return a + b;
}

fn max(a: int, b: int): int {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn sum_to_n(n: int): int {
    let sum: int = 0;
    for i = 1 to n {
        sum = sum + i;
    }
    return sum;
}

fn main() {
    let a: int = 5;
    let b: int = 10;
    let result_add: int = add(a, b);
    let result_max: int = max(a, b);
    let result_sum: int = sum_to_n(10);

    println("Addition of ", a, " and ", b, " is: ", result_add);
    println("Maximum of ", a, " and ", b, " is: ", result_max);
    println("Sum of numbers from 1 to 10 is: ", result_sum);
}
```