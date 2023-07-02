from delays import DelayManager
from timebase_delays import TimebaseDelays, place_event_on_timeline


def test_place_event_on_timeline():
    dm = DelayManager.from_list([
        ('e1', 'tb1', 'tb2', 50),
        ('e2', 'tb1', 'tb2', 100),
    ])

    tbd = TimebaseDelays.from_list(dm, [
        ('e1', 'tb1', 25),
        ('e1', 'tb2', 1075),
    ])
    tbd.build_timebase_adj_list()

    assert place_event_on_timeline(dm, tbd, 'e2', 'tb2', 1575, 'tb1') == 475
