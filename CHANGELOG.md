# v4.0.0 (2022-01-29)
- breaking changes to public API
- much better documentation
- API changes enable the easier selection of hashing algorithms
  in bins that use this lib. See example `algorithm_selection`.
- MSRV is 1.56.1 stable

# v3.0.1 (2022-01-27)
- massive performance improvements (20-40%)
- internal code quality improved

# v3.0.0 (2021-10-19)

- started CHANGELOG file
- several changes to public interface
  - all hashing algorithms now use a common trait and are no longer
    copied to a string
- multiple internal improvements
- better code quality
- slightly better performance
- MSRV is 1.52.1
