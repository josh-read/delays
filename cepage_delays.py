from delays import *


ids = InformationDelays.from_list([
    ('trig', 'berkley box', 'head scope', 8.2),
    ('current start', 'experiment', 'head scope', 65.5),
    ('xray peak', 'experiment', 'head scope', 330),
    ('light', 'experiment', 'pdv scope', 130),
    ('ext out', 'head scope', 'pdv scope', 104),
    ('sib mon', 'experiment', 'shimadzu', 10),
    ('sib mon', 'shimadzu', 'head scope', 100),
    ('ak mon', 'experiment', 'shimadzu', 10),
    ('ak mon', 'shimadzu', 'head scope', 75),
])

em = EventManager.from_timeline(ids, {
    'berkley box': {'trig': 0},
    'experiment': {'current start': 0},
    'head scope': {'trig': -895, 'ext out': 35, 'current start': 2500},
    'pdv scope': {'ext out': 0},
})
em.build_timebase_adj_list()
tbds = em.timebase_delays

pdv_timebase_to_exp = place_event_on_timeline(ids, tbds, 'light', 'pdv scope', 0, 'experiment')
x_ray_peak = place_event_on_timeline(ids, tbds, 'xray peak', 'head scope', 4320, 'experiment')
sib_shim_to_exp = place_event_on_timeline(ids, tbds, 'sib mon', 'head scope', 2700, 'experiment')
ak_shim_to_exp = place_event_on_timeline(ids, tbds, 'ak mon', 'head scope', 2700, 'experiment')
print(pdv_timebase_to_exp, x_ray_peak, sib_shim_to_exp, ak_shim_to_exp)
