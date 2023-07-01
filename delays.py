from collections import defaultdict
from pprint import pprint


class DelayManager:

    def __init__(self):
        self.delays = []
        self.delay_adjacency_dict = defaultdict(lambda: defaultdict(dict))

    def add(self, event, timebase1, timebase2, delay):
        """Getting the sign right on the delay is vital here. Should read like:

        information from event on timebase1 occurs delay later on timebase 2
        t2 - t1 = delay"""
        self.delays.append((event, timebase1, timebase2, delay))
        self.delay_adjacency_dict[event][timebase1].update({timebase2: delay})
        self.delay_adjacency_dict[event][timebase2].update({timebase1: -delay})


def main():
    dm = DelayManager()
    dm.add('e1', 'tb1', 'tb2', 100)
    dm.add('e1', 'tb2', 'tb3', 25)
    dm.add('e1', 'tb4', 'tb3', 250)
    dm.add('e1', 'tb1', 'tb4', 70)
    dm.add('e2', 'tb1', 'tb4', 140)
    pprint(dm.delay_adjacency_dict)


if __name__ == '__main__':
    main()
