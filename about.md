# About

## Introduction

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

## Theory

### Manual Method

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

We need 5 variables to work out a sixth.

Lets apply this to a slightly more complicated example.
This time we won't attribute real events to the events, we'll just work with symbols.

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

Here we are working with three timebases and we want to know at what time C appears on the first timebase.

We can write nine expressions linking all timebases and events together:

*Write these equations out or do it shorthand*

We can then solve the system of equations:

*Write as a matrix*

Even for this relatively simple case, this is really hard work.
It has a computational complexity of O(n^3)

### General method

Fortunately there is a more general method for these calculations by representing everything as a graph (network) of events connected by delays.
Each unique combination of event and timebase is represented by a node in the graph.
Each delay is represented by an edge whose weight is equal to the length of the delay.
Times of events are represented as a delay between a t0 node and the event node on the same timebase.

We can then calculate the delay between any two connected nodes (timebase-event pairs) on the graph by finding a path between them and summing the edge weights along the path.
Hence if we want the delay between two timebases we find the path between the t0 of each timebase.
If we want the information delay between two timebases for an event, we find the delay between the event on each timebase.
If we want the time of an event we find the delay between t0 on that timebase and the event.

We still require the same amount of information about either timebase offsets, delays or times, but now there is just one algorithm for calculating any information we wish.

## Under the hood

Graph traversal algorthms are efficient and scale well.
In addition graph theory gives us some useful concepts for figuring out what we need to measure to get the values we want.
There are great graph libraries in almost every language which means theres well tested and optimised stuff already existing rather than having to develop new algorithms from scratch.

The core rust library then just provides an abstraction layer around a graph.
The abstraction maps the concept of multiple timelines to the graph.
Rust was chosen because of its speed and safety

The web app...

The python library... and additionally adds plotting functionality

For examples on how to use each of these components, see the tutorial.
