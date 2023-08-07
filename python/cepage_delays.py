from delays import *
from pprint import pprint


INFO_DELAYS = INFO_DELAYS = InformationDelays.from_list([
    ('trig', 'berkley box', 'head scope', 8.2),
    ('current start', 'experiment', 'head scope', 65.5),
    ('xray peak', 'experiment', 'head scope', 330),
    ('light', 'experiment', 'pdv scope', 130),
    ('ext out', 'head scope', 'pdv scope', 104),
    ('sib mon', 'experiment', 'shimadzu', 8),
    ('sib mon', 'shimadzu', 'head scope', 77),
    ('ak mon', 'experiment', 'shimadzu', 7),
    ('ak mon', 'shimadzu', 'head scope', 50.5),
])


def optical_conf(current_start_time, sib_mon_time, ak_mon_time):
    em = EventManager.from_timeline(INFO_DELAYS, {
        'berkley box': {'trig': 0},
        'experiment': {'current start': 0},
        'head scope': {'trig': -895, 'ext out': 35, 'current start': current_start_time},
        'pdv scope': {'ext out': 0},
    })
    em.build_timebase_adj_list()
    tbds = em.timebase_delays

    return {
        'pdv_timebase_to_exp': place_event_on_timeline(INFO_DELAYS, tbds, 'light', 'pdv scope', 0, 'experiment'),
        'sib_shim_to_exp': place_event_on_timeline(INFO_DELAYS, tbds, 'sib mon', 'head scope', sib_mon_time, 'experiment'),
        'ak_shim_to_exp': place_event_on_timeline(INFO_DELAYS, tbds, 'ak mon', 'head scope', ak_mon_time, 'experiment'),
    }


def xray_conf(current_start_time, sib_mon_time, xray_time):

    em = EventManager.from_timeline(INFO_DELAYS, {
        'berkley box': {'trig': 0},
        'experiment': {'current start': 0},
        'head scope': {'trig': -895, 'ext out': 35, 'current start': current_start_time},
        'pdv scope': {'ext out': 0},
    })
    em.build_timebase_adj_list()
    tbds = em.timebase_delays

    return {
        'pdv_timebase_to_exp': place_event_on_timeline(INFO_DELAYS, tbds, 'light', 'pdv scope', 0, 'experiment'),
        'sib_shim_to_exp': place_event_on_timeline(INFO_DELAYS, tbds, 'sib mon', 'head scope', sib_mon_time, 'experiment'),
        'x_ray_peak': place_event_on_timeline(INFO_DELAYS, tbds, 'xray peak', 'head scope', xray_time, 'experiment'),
    }


if __name__ == '__main__':
    print('s0267')
    s0267 = optical_conf(
        current_start_time=2450, 
        sib_mon_time=-476, 
        ak_mon_time=-224)
    pprint(s0267)

    print('s0268')
    s0268 = optical_conf(
        current_start_time=2508,
        sib_mon_time=-468,
        ak_mon_time=-171
    )
    pprint(s0268)

    print('s0270')
    s0270 = xray_conf(
        current_start_time=2520,
        sib_mon_time=1492,
        xray_time=(4891 + 4999)/2,
    )
    pprint(s0270)

    print('s0271')
    s0271 = xray_conf(
        current_start_time=2511,
        sib_mon_time=1478,
        xray_time=(5507 + 5607)/2,
    )
    pprint(s0271)

    print('s0272')
    s0272 = xray_conf(
        current_start_time=2498,
        sib_mon_time=1472,
        xray_time=(7500+7650)/2,
    )
    pprint(s0272)
