# RustVU (CS 3891) - Homework 3

## Optional / warmup exercises for the week

I highly recommend the [Rustlings project](https://github.com/rust-lang/rustlings) for practicing the basic concepts we learn in this class. These completely optional, not graded/submitted exercises can help you to teach Rust programming to your "fingers".

The recommended exercises for this week:

- `error_handling`
- `modules`

## Assignment

This assignment contains a single Rust crate focusing on error handling and modules.

The program (`gps_track` crate) reads and analyses a GPS track file. The file format is a simple CSV (comma-separated values), where each line contains three floating point numbers in this order:

```csv
time,latitude,longitude
```

The `geo` module is responsible for handling lat/lon coordinates and to compute the distance (on the surface of Earth) between two coordinates.
I recommend to start with this module and its test cases.

The main program is supposed to read the CSV file and implement some computations on the dataset.

It is part of your assignment to research and learn some basic file reading operations in Rust.

One of the hardest part of error handling in Rust is to deal with multiple types of errors in a function and propagate errors across functions. I provided a custom error type (`TrackError`), which is capable to wrap all potential errors you may encounter in the assignment. You need to figure out how to use this facility.

Make sure that crate compiles and builds properly and passes all of the built-in tests.

I placed `// TODO` comments in the code where I expect you to add implementation code. The test code is clearly marked with a `// DO NOT EDIT BELOW THIS LINE` comment. This should be evident: changing the test code is a (not too intelligent) way of cheating. I will handle any such attempts accordingly. However, you are allowed and encouraged to look at the test code to better understand what is expected from you.

## Use

You can always check your work with `cargo test`. You can also run individual tests by running `cargo test <test-name>` (see the names below).

The crate contains a proper `main()` function. You can modify this function for development/debugging purposes. The output of `cargo run` is not tested or graded.

## Grading

Make sure you __commit__ and __push__ your assignment repository once you manage to run `cargo test` without any errors or warnings.

The homework is graded by test (no partial credits are given for failed tests):

|Test                            | Points|
|--------------------------------|-------|
|geo::tests::test_new            |    10 |
|geo::tests::test_derived_traits |     5 |
|geo::tests::test_distance_point |    20 |
|test_from_line                  |    20 |
|test_read_track                 |    15 |
|test_total_distance             |    10 |
|test_average_speed              |    10 |
|test_max_speed                  |    10 |

Once you __push__ your solution to the repository, GitHub Classroom will run the automated test. I highly recommend to [verify your results of this CI/CD worflow](https://docs.github.com/en/education/manage-coursework-with-github-classroom/learn-with-github-classroom/view-autograding-results) - I use these results for grading your work.
