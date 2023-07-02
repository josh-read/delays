from delays import DelayManager
import pytest


def test_no_route():
    dm = DelayManager.from_list([
        ('e1', 'tb1', 'tb2', 100),
        ('e1', 'tb4', 'tb3', 250),
    ])
    with pytest.raises(ValueError):
        dm.find_delay('e1', 'tb1', 'tb4')


def test_single_route():
    dm = DelayManager.from_list([
        ('e1', 'tb1', 'tb2', 100),
        ('e1', 'tb2', 'tb3', 25),
        ('e1', 'tb4', 'tb3', 250),
    ])
    delay = dm.find_delay('e1', 'tb1', 'tb4')
    assert delay == -125


def test_two_routes():
    dm = DelayManager.from_list([
        ('e1', 'tb1', 'tb2', 100),
        ('e1', 'tb2', 'tb3', 25),
        ('e1', 'tb4', 'tb3', 250),
        ('e1', 'tb1', 'tb4', -125),
    ])
    delay = dm.find_delay('e1', 'tb1', 'tb4')
    assert delay == -125
