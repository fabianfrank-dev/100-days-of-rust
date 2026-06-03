# Day 6: Traits, Generics & Libraries

## Projects
- **Traits**: Implemented `Add` and `Sub` for a custom `Point` type.
- **Generic Parameters**: Built a generic currency exchange example converting through USD.
- **Libraries**: Used the `rand` crate to generate random points and calculate distance.
- **Money**: Practiced conversion traits with `ToUSD`, `FromUSD`, and a reusable `convert` method.

## Learnings
✅ Implementing standard traits like `Add` and `Sub`
✅ Defining associated output types with `type Output`
✅ Using generic traits and `where` bounds
✅ Writing unit tests with `#[cfg(test)]`
✅ Adding external crates such as `rand`
✅ Using `.clone()` when values are needed after ownership moves

## Challenges
- **Generic conversions**: Practiced structuring conversion traits so different currencies can share behavior.
- **Ownership with operators**: Learned that custom `+` and `-` operations consume values unless they are cloned or borrowed differently.
