# delays

A Rust-based library for calculating experimental delays
with a Web App and Python bindings.

## Overview

This project aims to help scientists and engineers to manage data on multiple timebases
and sync event times up to any frame of reference.

This is useful when the transit time of information from the measurement frame to detectors is comparable to the length of the experiment itself.
This condition is often true for the very short lived experiments common in high energy density physics.

The tools offered here are designed to target two practical use cases:
- Setting delay pulse generators to trigger detectors at the right time to record over a time of interest (before the experiment).
- Syncing recorded data to a common frame of reference (after the experiment).

Examples of both use cases are presented in the tutorial.

## Features

- Core Rust Library:
  - Provides the Timelines struct, a user-friendly abstraction over a graph (network) of delays.
  - Timelines struct features chacked entry, such that it is impossible to overconstrain an event on a timebase.
  - Uses petgraph library for graph operations

- Web App:
  - User-friendly interface for inputting data and performing calculations.
  - Visualization of timebase conversions.
  - Export functionality for use in Python analysis.
<!--
- Python Library:
  - Compiled with PyO3 for seamless integration with Python.
  - Provides access to the core library's functionality for experimental analysis.
-->

## Getting Started

### Installation

- Rust library: install with cargo
- Web app: no installation required
- Python library: coming soon

### Usage

## About

### Introduction

Timelines are a common way of visualising a series of events and their chronological relationship to each other.

For example, imagine two events: A and B.
Event A is a clap of sound while B is a flash of light.

A timeline can be drawn to illustrate these events.

```
     A     B
|----|-----|---->
    100   200
```

In a different frame of reference some distance away, the exact same series of events may appear as:

```
     B     A
|----|-----|---->
    100   200
```

We'll assume for this example that light travels instantaneously.

Then why has the time of event B changed?

This is because the origin of the timeline can be different, in this case there is a timebase delay of 100 between the first and the second timeline.

The order of A and B has also changed because the flash of light travels faster than the clap of sound.

In this case we've observed an "information delay" of 200 for the clap of sound from the first frame of reference to the second.

We can make this clear by drawing both timelines together:

```
     A     B
|----|-----|---->
    100   200
       \   |
         \ |
           |
           | \
           |   \
           B     A
      |----|-----|---->
          100   200
```

In HEDP experiments...
- short timescales
- many different detectors
- signals in cables, light in fibres, light in free space
- things get complicated so we would like to use a tool

### Theory

In the introductory example, we calculated the information delay for event A between the two frames of reference.

We can define some terms and write this more formally:

- $t_{A1}$: Time of event A on timebase 1
- $t_{B1}$: Time of event B on timebase 1
- $t_{A2}$: Time of event A on timebase 2
- $t_{B2}$: Time of event A on timebase 2
- $\Delta t_{A12}$: Information delay between timebase 1 and 2 for event A
- $\Delta t_{B12}$: Information delay between timebase 1 and 2 for event B

We can link all of these quantities together in one equation.
To derive it more easily lets intoduce another term $\Delta t_{12}$,
which is the delay between the first and second timebase.
In the previous example this would've been 100.
Each event is linked between timebases in the same way:

- ① : $t_{A2} - t_{A1} = \Delta t_{A12} + \Delta t_{12}$
- ② : $t_{B2} - t_{B1} = \Delta t_{B12} + \Delta t_{12}$
- ① - ② : $(t_{A2} - t_{A1}) - (t_{B2} - t_{B1}) = \Delta t_{A12} - \Delta t_{B12}$

We can apply this to the previous example to obtain the information delay for A between
the timebases.

$$(t_{A2} - t_{A1}) - (t_{B2} - t_{B1}) = \Delta t_{A12} - \Delta t_{B12}$$

$$(200 - 100) - (100 - 200) = \Delta t_{A12} - 0$$

$$\Delta t_{A12} = 200$$

This is simple enough and gives the correct result.
Now, lets look at a more complex example.

```
     A     C
|----|-----|------>
    100    ?
       \     \
     100 \     \
           \     \
      B     A      \
 |----|-----|---->   \ 300
     100   200         \
        \                \
          \ 50             \
           B                 C
 |---------|-----------------|---->
          200               500
```

Here we are working with three timebases and we want to know at what time C appears on the first timebase. Even for this still relatively simple case, this is really hard work.

<!-- We can write nine expressions linking all timebases and events together:

For timebases 1 and 2:

$$(t_{A2} - t_{A1}) - (t_{B2} - t_{B1}) - \Delta t_{A12} + \Delta t_{B12} = 0$$
$$(t_{A2} - t_{A1}) - (t_{C2} - t_{C1}) - \Delta t_{A12} + \Delta t_{C12} = 0$$
$$(t_{B2} - t_{B1}) - (t_{C2} - t_{C1}) - \Delta t_{B12} + \Delta t_{C12} = 0$$

1 and 3:

$$(t_{A3} - t_{A1}) - (t_{B3} - t_{B1}) - \Delta t_{A13} + \Delta t_{B13} = 0$$
$$(t_{A3} - t_{A1}) - (t_{C3} - t_{C1}) - \Delta t_{A13} + \Delta t_{C13} = 0$$
$$(t_{B3} - t_{B1}) - (t_{C3} - t_{C1}) - \Delta t_{B13} + \Delta t_{C13} = 0$$

2 and 3:

$$(t_{A3} - t_{A2}) - (t_{B3} - t_{B2}) - \Delta t_{A23} + \Delta t_{B23} = 0$$
$$(t_{A3} - t_{A2}) - (t_{C3} - t_{C2}) - \Delta t_{A23} + \Delta t_{C23} = 0$$
$$(t_{B3} - t_{B2}) - (t_{C3} - t_{C2}) - \Delta t_{B23} + \Delta t_{C23} = 0$$

We have 18 symbols here, we know the value of 8 of them from the problem definition, and we have 9 equations. We therefore have enough information to get the value of every symbol.

We can express the problem as a matrix:

$$
\begin{bmatrix}
-1 & 1 & 0 & 1 & -1 & 0 & 0 & 0 & 0 & -1 & 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
-1 & 1 & 0 & 0 & 0 & 0 & 1 & -1 & 0 & -1 & 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 \\
0 & 0 & 0 & -1 & 1 & 0 & 1 & -1 & 0 & 0 & 0 & 0 & -1 & 0 & 0 & 1 & 0 & 0 \\

-1 & 0 & 1 & 1 & 0 & -1 & 0 & 0 & 0 & 0 & -1 & 0 & 0 & 1 & 0 & 0 & 0 & 0 \\
-1 & 0 & 1 & 0 & 0 & 0 & 1 & 0 & -1 & 0 & -1 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\
0 & 0 & 0 & -1 & 0 & 1 & 1 & 0 & -1 & 0 & 0 & 0 & 0 & -1 & 0 & 0 & 1 & 0 \\

0 & -1 & 1 & 0 & 1 & -1 & 0 & 0 & 0 & 0 & 0 & -1 & 0 & 0 & 1 & 0 & 0 & 0 \\
0 & -1 & 1 & 0 & 0 & 0 & 0 & 1 & -1 & 0 & 0 & -1 & 0 & 0 & 0 & 0 & 0 & 1 \\
0 & 0 & 0 & 0 & -1 & 1 & 0 & 1 & -1 & 0 & 0 & 0 & 0 & 0 & -1 & 0 & 0 & 1 \\

1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 & 0 & 0 \\
0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 0 & 1 & 0 \\
\end{bmatrix}
\cdot
\begin{bmatrix}
t_{A1} \\
t_{A2} \\
t_{A3} \\
t_{B1} \\
t_{B2} \\
t_{B3} \\
t_{C1} \\
t_{C2} \\
t_{C3} \\
\Delta t_{A12} \\
\Delta t_{A13} \\
\Delta t_{A23} \\
\Delta t_{B12} \\
\Delta t_{B13} \\
\Delta t_{B23} \\
\Delta t_{C12} \\
\Delta t_{C13} \\
\Delta t_{C23} \\
\end{bmatrix}
=
\begin{bmatrix}
0 \\
0 \\
0 \\
0 \\
0 \\
0 \\
0 \\
0 \\
0 \\
100 \\
200 \\
100 \\
200 \\
? \\
500 \\
100 \\
50 \\
300 \\
\end{bmatrix}
$$ -->

Fortunately there is an easier way than defining symbols and deriving relationships between each event and timebase. Instead, we can see from the graph that there is a path between $t_{C1}$ and $t_{A1}$ which *is* referenced to timebase 1. By recognising that the time of each event is equivalent to a delay from the origin of each timebase, we can just sum the delays from $t_{C1}$ and $t_{01}$.

The sign of the delays is extremely important here, and it this method is error prone when calculating it manually as we do here:

$$t_{C1} \rightarrow t_{C3} \rightarrow t_{B3} \rightarrow t_{B2} \rightarrow t_{A2} \rightarrow t_{A1}$$

$$-300 + 300 + 50 -100 + 100 + 100 = 150$$

We can formalise this approach by describing the system as a graph (network) of events connected by delays.
Each unique combination of event and timebase is represented by a node in the graph.
Each delay is represented by an edge whose weight is equal to the length of the delay.
Times of events are represented as a delay between a t0 node and the event node on the same timebase.

We can then calculate the delay between any two connected nodes (timebase-event pairs) on the graph by finding a path between them and summing the edge weights along the path.
Hence if we want the delay between two timebases we find the path between the t0 of each timebase.
If we want the information delay between two timebases for an event, we find the delay between the event on each timebase.
If we want the time of an event we find the delay between t0 on that timebase and the event.

We still require the same amount of information about either timebase offsets, delays or times, but now there is just one algorithm for calculating any information we wish.

### Under the hood

Graph traversal algorthms are efficient and scale well.
In addition, graph theory gives us some useful concepts for figuring out what we need to measure to get the values we want.
There are great graph libraries in almost every language which means theres well tested and optimised stuff already existing rather than having to develop new algorithms from scratch.

The core rust library then just provides an abstraction layer around a graph.
The abstraction maps the concept of multiple timelines to the graph.
Rust was chosen because of its speed and safety.

The web app provides a user friendly interface for data entry and delay calculations.
It is also written in Rust, but compiles to a website backed by web assembly,
allowing it to be portable while still running at near native speeds.

A Python wrapper is also planned which will provide bindings for the core Rust library.
Since Python is commonly used by scientists, it will allow for the delays
tool to integrate into existing data analysis pipelines.

## Tutorial

### Calculating trigger delays

### Syncing data to common timebase
