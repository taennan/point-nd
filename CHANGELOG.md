
# Changelog

## 0.5.0

- Removed compulsory ```Copy``` trait bounds
- Removed compulsory ```Default``` trait bounds
- Renamed ```from()``` constructor to a more descriptive ```from_slice()```
- Changed generics in ```apply_vals()``` method to accept any type that implements ```Clone```
- Removed ```into_vec()``` method for ```no_std``` compatibility

