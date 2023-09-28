# About

## Introduction

Timelines are a common way of visualising a series of events and their chronological relationship to each other.

For example:
```
     A     B
|----|-----|---->
    100   200
```
Now in a different frame of reference the exact same series of events may appear as:
```
     B     A
|----|-----|---->
    100   200
```
How? 
Well the t<sub>0</sub> of each timeline are not nescessarily synchronised. 
Also it may take longer for the information from event A to reach the second timebase than it does for event B so the order swaps around.

For example event A may be a sound and event B may be visual.
Because light travels faster than sound, it appears to the observer on the second timebase that the flash occurs before the clap.

In HEDP experiments...
- short timescales
- many different detectors
- signals in cables, light in fibres, light in free space
- things get complicated so we would like to use a tool

## Theory

### Manual Method

Going back to the first example, if we know the distance between the two observers then we can calculate the difference between the two timebases.
In this example d=...
This is an atypical example, normally we would just measure the transit time of a cable or fibre, or the optical path length of light in free space.

Now that we know the difference between the timebases, if there was a third event C on either timebase, and we knew the amount of time it takes for the signal to travel from the first frame of reference to the second, we could calculate the time it occurs on the second timebase.
Similarly, if we have the time of the event on both timebases, we can calculate the time it takes for the signal to travel between the timebases.

To summarise, given three of the following:
- time on timebase 1
- time on timebase 2
- information delay between the timebases
- delay between the timebases
You can calculate the fourth.

This method works fine, but quickly becomes complicated with many timebases and is prone to sign errors.

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
