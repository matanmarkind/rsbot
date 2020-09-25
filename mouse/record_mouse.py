"""
DEPRECATED, USE RUST VERSION.

This file is used to track the mouses location and record it out to a file.

Output colums in order are:
1. dt - time between samplings in us (0 = hard break)
2. x - absolute horizontal location in pixels (from top left)
3. y - absolute vertical location in pixels (from top left)
"""
import sys
import time
import csv
import mouse
import statistics

from collections import namedtuple

from absl import app
from absl import flags
from absl import logging

FLAGS = flags.FLAGS

flags.DEFINE_string('ofpath', None, 'Path to output file')
# 'sampling_period' offers a tradeoff. The higher rate creates more mouselike movement to avoid teleporting
# mouse detection. The downside is that the recording takes time so the shorter we make this the less
# stable the actual sampling period becomes. We face the same problem on the replay end, since the usage
# has to be able to keep up with the rate being fed in. It also means more data so more memory.
flags.DEFINE_float('sampling_period', 0.01,
                   'Minimum wait time between recording mouse position (seconds). get_location seem to take a bit under 1ms.')
flags.DEFINE_float('batch_period', 1,
                   'Period between writing out recordings to a file (seconds).')
flags.DEFINE_float('active_time', 2,
                   'Period between writing out recordings to a file (seconds).')


def setup_logging():
    # Write logs to file instead of stderr.
    logging.get_absl_handler().use_absl_log_file()
    # Write logs that are ERROR or more severe also to the terminal.
    logging.set_stderrthreshold(logging.ERROR)


def main(argv):
    del argv  # Unused
    setup_logging()

    Row = namedtuple('Row', ['dt', 'x', 'y'])
    start_time = time.time()
    while time.time() < (start_time + FLAGS.active_time):
        batch_period = FLAGS.batch_period
        sampling_period = FLAGS.sampling_period
        batch_iters = int(batch_period * 1/sampling_period)
        rows = [Row(0, 0, 0)] * batch_iters

        with open(FLAGS.ofpath, 'a') as f:
            writer = csv.writer(f)
            t = time.time()
            # Start at 1 instead of 0, to leave the first element as (0, 0, 0).
            # This is used to indicate a break.
            for i in range(1, batch_iters):
                time.sleep(max(sampling_period - (time.time() - t), 0))

                loc = mouse.get_position()
                t = time.time()
                rows[i] = Row(int(t * 1e6), loc[0], loc[1])

            writer.writerows(rows)

        times = [r.dt for r in rows]
        summary = (min(times), max(times), statistics.mean(
            times), statistics.stdev(times))
        print([s for s in summary])


if __name__ == '__main__':
    app.run(main)
