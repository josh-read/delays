from delays import InformationDelays, EventManager, place_event_on_timeline
import pytest


def test_no_route():
    dm = InformationDelays.from_list([
        ('e1', 'tb1', 'tb2', 100),
        ('e1', 'tb4', 'tb3', 250),
    ])
    with pytest.raises(ValueError):
        dm.find_delay('e1', 'tb1', 'tb4')


def test_single_route():
    dm = InformationDelays.from_list([
        ('e1', 'tb1', 'tb2', 100),
        ('e1', 'tb2', 'tb3', 25),
        ('e1', 'tb4', 'tb3', 250),
    ])
    delay = dm.find_delay('e1', 'tb1', 'tb4')
    assert delay == -125


def test_two_routes():
    dm = InformationDelays.from_list([
        ('e1', 'tb1', 'tb2', 100),
        ('e1', 'tb2', 'tb3', 25),
        ('e1', 'tb4', 'tb3', 250),
        ('e1', 'tb1', 'tb4', -125),
    ])
    delay = dm.find_delay('e1', 'tb1', 'tb4')
    assert delay == -125


def test_place_event_on_timeline():
    ids = InformationDelays.from_list([
        ('e1', 'tb1', 'tb2', 50),
        ('e2', 'tb1', 'tb2', 100),
    ])

    em = EventManager.from_timeline(ids, {
        'tb1': {'e1': 25},
        'tb2': {'e1': 1075},
    })
    em.build_timebase_adj_list()
    tbds = em.timebase_delays

    assert place_event_on_timeline(ids, tbds, 'e2', 'tb2', 1575, 'tb1') == 475
