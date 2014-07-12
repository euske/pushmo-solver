#!/usr/bin/env python
import sys

MAX = 3
MOVES = [(-1,+1),(0,+1),(+1,+1), (-1,0),(+1,0), (-1,-1),(0,-1),(+1,-1)]

class Segment(object):

    def __init__(self, name):
        self.name = name
        self.locs = []
        return

    def __repr__(self):
        return ('<Segment %s: %r>' % (self.name, self.locs))

    def add(self, p):
        self.locs.append(p)
        return

class Board(object):

    def __init__(self, rows):
        self.width = len(rows[0])
        self.height = len(rows)
        self.goal = None
        self.segments = []
        self.loc2seg = {}
        d = {}
        for (y,row) in enumerate(reversed(rows)):
            for (x,c) in enumerate(row):
                if c == '*':
                    self.goal = (x,y)
                if not c.isalnum(): continue
                if c in d:
                    i = d[c]
                else:
                    i = len(self.segments)
                    seg = Segment(c)
                    self.segments.append(seg)
                    d[c] = i
                p = (x,y)
                self.loc2seg[p] = i
                self.segments[i].add(p)
        return

    def getseg(self, loc):
        return self.loc2seg.get(loc)

    def show(self, loc):
        for y in xrange(self.height, -1, -1):
            row = []
            for x in xrange(0, self.width):
                if loc == (x,y):
                    row.append('@')
                elif self.goal == (x,y):
                    row.append('*')
                else:
                    i = self.getseg((x,y))
                    if i is None:
                        row.append('.')
                    else:
                        row.append(self.segments[i].name)
            print ' '+''.join(row)
        return

class Config(object):

    def __init__(self, board, depths):
        self.board = board
        self.depths = depths
        return

    def show(self, loc):
        for y in xrange(self.board.height, -1, -1):
            row = []
            for x in xrange(0, self.board.width):
                if loc == (x,y):
                    row.append('@')
                elif self.board.goal == (x,y):
                    row.append('*')
                else:
                    i = self.board.getseg((x,y))
                    if i is None:
                        row.append('.')
                    else:
                        row.append(str(self.depths[i]))
            print ' '+''.join(row)
        return

    def getdepth(self, loc):
        (x,y) = loc
        if y < 0:
            return MAX+1
        i = self.board.getseg(loc)
        if i is None:
            return 0
        else:
            return self.depths[i]

    def getsegs(self, loc):
        (x,y) = loc
        z = self.getdepth((x,y))
        # ground block.
        s0 = self.board.getseg((x,y-1))
        z0 = self.getdepth((x,y-1))
        assert z < z0, (self.depths, loc)
        r = []
        # front block.
        sf = self.board.getseg((x,y))
        if sf is not None and sf != s0 and z < z0-1:
            r.append((sf, 0, z0-1))
        # side block. (left)
        sl = self.board.getseg((x-1,y))
        zl = self.getdepth((x-1,y))
        if sl is not None and sl != s0 and z < zl:
            r.append((sl, z+1, MAX))
        # side block. (right)
        sr = self.board.getseg((x+1,y))
        zr = self.getdepth((x+1,y))
        if sr is not None and sr != s0 and z < zr:
            r.append((sr, z+1, MAX))
        return r

    def getlocs(self, loc):
        locs = set()
        def expand(p):
            if p in locs: return
            locs.add(p)
            (x0,y0) = p
            z0 = self.getdepth((x0,y0))
            z1 = self.getdepth((x0,y0-1))
            for (dx,dy) in MOVES:
                (x1,y1) = (x0+dx,y0+dy)
                if x1 < 0 or self.board.width <= x1 or y1 < 0: continue
                if 0 < dy:
                    # jumping.
                    if (self.getdepth((x1,y1)) < z0 and
                        z0 <= self.getdepth((x1,y1-1))):
                        expand((x1,y1))
                else:
                    # walking/falling.
                    if (self.getdepth((x1,y1)) < z1 and
                        z0 < self.getdepth((x1,y1-1))):
                        expand((x1,y1))
        expand(loc)
        return locs

def solve_pushmo(board, startloc, verbose=False):
    depths = tuple( 0 for _ in board.segments )
    queue = []
    queue.append((None, depths, startloc))
    states = {}
    solution = None
    while queue:
        queue.sort()
        prev = queue.pop(0)
        (_, depths, loc0) = prev
        if depths in states:
            locsets = states[depths]
        else:
            locsets = states[depths] = []
        found = False
        for locs in locsets:
            if loc0 in locs:
                found = True
                break
        if found: continue
        config = Config(board, depths)
        newlocs = config.getlocs(loc0)
        if verbose:
            print '-- Move %d --' % n
            config.show(loc0)
            print ' Possible locations:', sorted(newlocs)
            print
        if board.goal in newlocs:
            solution = (prev, depths, board.goal)
            break
        locsets.append(newlocs)
        for loc in newlocs:
            for (i,z0,z1) in config.getsegs(loc):
                for z in xrange(z0, z1+1):
                    d = depths[:i] + (z,) + depths[i+1:]
                    if d in states: continue
                    queue.append((prev, d, loc))
    r = []
    while solution is not None:
        (prev, depths, loc) = solution
        r.append((depths,loc))
        solution = prev
    r.reverse()
    return r

def main(argv):
    import fileinput
    board = Board(list(fileinput.input()))
    startloc = (0,0)
    print '-- Initial state --'
    board.show(startloc)
    print
    steps = solve_pushmo(board, startloc)
    if not steps:
        print 'Unsolvable.'
    else:
        for (i,(depths, loc)) in enumerate(steps):
            config = Config(board, depths)
            print '-- Move %d --' % i
            config.show(loc)
            print
        print 'Solved in %d steps.' % len(steps)
    return 0

if __name__ == '__main__': sys.exit(main(sys.argv))
