/**
 * By dividing f(n)=n2+n+41 by a couple of the valid ns I realized that f(1)=f(-2), f(2)=f(-3) .. f(x)=f(-(x+1))
 * So in reality f(n) generates primes for -40<=n<=39
 * However g(n)=n2-79n+1601 has valid solutions for n<=79
 * By dividing g(n) by a couple of the valid ns I noticed that g(1)=g(78), g(2)=g(77) .. g(x)=g(79-x)
 * I tried for a similar pattern for f(n) and suddenly, f(39-n)=g(n)
 * Notice the pattern that the prime-generating quadratic f(n) provides valid answers for n<=39 and f(39-n) is a prime-generating quadratic too
 * So we should find the biggest k such that f(k-n)=n^2+an+b where |a|,|b|<1000
 * The first k that satisfies this condition is k-30 were h(x)=f(30-x)=x2-61x+971
 */

fn main() {
    println!("{}", -61*971);
}