import itertools
from collections import defaultdict
from delays import DelayManager
from graph import graph_search
from pprint import pprint
from statistics import mean


class TimebaseDelays:

    def __init__(self, delay_manager):
        self.events = defaultdict(dict)
        self.timebase_adjacency_dict = defaultdict(dict)
        self.delay_manager = delay_manager

    @classmethod
    def from_list(cls, delay_manager, event_list):
        timebase_delays = cls(delay_manager)
        for row in event_list:
            timebase_delays.add_event(*row)
        return timebase_delays

    def add_event(self, event, timebase, time):
        """Assumes for now that all delays are added ahead of time."""
        self.events[event][timebase] = time

    def build_timebase_adj_list(self):
        for event, timebases in self.events.items():
            timebase_combos = itertools.combinations(timebases, r=2)
            for tb1, tb2 in timebase_combos:
                try:
                    information_delay = self.delay_manager.find_delay(event, tb1, tb2)
                except ValueError:
                    continue
                else:
                    total_delay = self.events[event][tb2] - self.events[event][tb1]
                    timebase_delay = total_delay - information_delay
                    self.timebase_adjacency_dict[tb1][tb2] = timebase_delay
                    self.timebase_adjacency_dict[tb2][tb1] = -timebase_delay

    def find_delay(self, timebase1, timebase2):
        graph_search_result = graph_search(self.timebase_adjacency_dict, timebase1, timebase2)
        if len(graph_search_result) == 0:
            raise ValueError
        try:
            (_, delay), = graph_search_result
            return delay
        except ValueError:
            print(f'Found {len(graph_search_result)} possible ways of linking from {timebase1} to {timebase2}:')
            pprint([path for path, _ in graph_search_result])
            delays = [delay for _, delay in graph_search_result]
            print(f'Taking the average delay from {delays}.')
            return mean(delays)


def place_event_on_timeline(delay_manager, timebase_delays, event, current_timeline, time, target_timeline):
    information_delay = delay_manager.find_delay(event, current_timeline, target_timeline)
    timebase_delay = timebase_delays.find_delay(current_timeline, target_timeline)
    return time + information_delay + timebase_delay
