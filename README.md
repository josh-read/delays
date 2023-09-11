# delays

A Rust-based library for calculating experimental delays
with a Web App and Python bindings.

## Overview

This project aims to help scientists and engineers to manage data on multiple timebases
and sync event times up to any frame of reference.

This is useful when either of the following is true:
- The detectors used to collect data can only run for a short amount of time relative to the duration of the experiment.
- The transit time of information around the system is comparable to the length of the experiment itself.
These conditions are both typically true for very short lived experiments which are common in HEDP.

The tools offered here are designed to target two key scenarios:
- Setting delay pulse generators to trigger detectors at the right time to record over a time of interest.
- Syncing data to the experimental frame of reference.
