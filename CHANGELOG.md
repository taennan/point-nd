
# Changelog

## 0.5.0

- Removed compulsory ```Default``` trait bounds
- Removed compulsory ```Clone``` trait bounds
- Removed compulsory ```Copy``` trait bounds
- Renamed ```from()``` constructor to a more descriptive ```from_slice()```
- Changed generics in ```apply_vals()``` method to accept arrays with items of any type
- Changed generics in ```apply_point()``` method to accept ```PointND```'s with items of any type
- Removed ```into_vec()``` method for ```no_std``` compatibility

