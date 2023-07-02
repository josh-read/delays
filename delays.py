from collections import defaultdict
from itertools import combinations
from pprint import pprint
from statistics import mean

from graph import graph_search
from utils import transpose_nested_dict


class DelayGraph:

    def __init__(self):
        self.adjacency_dict = defaultdict(dict)

    def add(self, start_timebase, end_timebase, delay):
        """Add nodes to the delay graph.

        Getting the sign right on the delay is vital here.

        t_end - t_start = delay"""
        self.adjacency_dict[start_timebase][end_timebase] = delay
        self.adjacency_dict[end_timebase][start_timebase] = -delay

    def find_delay(self, start_timebase, end_timebase):
        graph_search_result = graph_search(self.adjacency_dict, start_timebase, end_timebase)
        if len(graph_search_result) == 0:
            raise ValueError
        elif len(graph_search_result) == 1:
            (_, delay), = graph_search_result
            return delay
        elif len(graph_search_result) >= 2:
            print(f'Found {len(graph_search_result)} possible ways of linking '
                  f'{start_timebase} to {end_timebase}:')
            pprint([path for path, _ in graph_search_result])
            delays = [delay for _, delay in graph_search_result]
            print(f'Taking the average delay from {delays}.')
            return mean(delays)


class InformationDelays:

    def __init__(self):
        self.event_delay_graphs = defaultdict(DelayGraph)

    @classmethod
    def from_list(cls, delay_list):
        delay_manager = cls()
        for row in delay_list:
            delay_manager.add(*row)
        return delay_manager

    def add(self, event, timebase1, timebase2, delay):
        """Getting the sign right on the delay is vital here. Should read like:

        information from event on timebase1 occurs delay later on timebase 2
        t2 - t1 = delay"""
        self.event_delay_graphs[event].add(timebase1, timebase2, delay)

    def find_delay(self, event, timebase1, timebase2):
        return self.event_delay_graphs[event].find_delay(timebase1, timebase2)


class EventManager:

    def __init__(self, information_delays, events):
        self.events = events
        self.information_delays = information_delays
        self.timebase_delays = TimebaseDelays()

    @classmethod
    def from_timeline(cls, information_delays, timeline):
        event_manager = cls(information_delays, transpose_nested_dict(timeline))
        return event_manager

    def build_timebase_adj_list(self):
        for event, timebases in self.events.items():
            timebase_combos = combinations(timebases, r=2)
            for tb1, tb2 in timebase_combos:
                try:
                    information_delay = self.information_delays.find_delay(event, tb1, tb2)
                except ValueError:
                    continue
                else:
                    total_delay = self.events[event][tb2] - self.events[event][tb1]
                    timebase_delay = total_delay - information_delay
                    self.timebase_delays.add(tb1, tb2, timebase_delay)


class TimebaseDelays(DelayGraph):
    pass


def place_event_on_timeline(delay_manager, timebase_delays, event, current_timeline,
                            time, target_timeline):
    information_delay = delay_manager.find_delay(event, current_timeline,
                                                 target_timeline)
    timebase_delay = timebase_delays.find_delay(current_timeline, target_timeline)
    return time + information_delay + timebase_delay
