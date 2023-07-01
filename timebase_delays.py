import itertools
from collections import defaultdict
from delays import DelayManager


class TimebaseDelays:

    def __init__(self, delay_manager):
        self.events = defaultdict(dict)
        self.timebase_adjacency_list = defaultdict(dict)
        self.delay_manager = delay_manager

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
                    self.timebase_adjacency_list[tb1][tb2] = timebase_delay
                    self.timebase_adjacency_list[tb2][tb1] = -timebase_delay


def main():
    dm = DelayManager()
    dm.add('e1', 'tb1', 'tb2', 100)
    dm.add('e1', 'tb2', 'tb3', 25)
    dm.add('e1', 'tb4', 'tb3', 250)
    dm.add('e1', 'tb1', 'tb4', 70)
    dm.add('e2', 'tb1', 'tb4', 140)

    tbd = TimebaseDelays(dm)
    tbd.add_event('e1', 'tb1', 0)
    tbd.add_event('e1', 'tb4', 600)
    print(tbd.build_timebase_adj_list())
    print(tbd.timebase_adjacency_list)



if __name__ == '__main__':
    main()
