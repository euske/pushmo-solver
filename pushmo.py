#!/usr/bin/env python
import sys

INF = sys.maxint

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
        self.start = None
        self.goal = None
        self.segments = []
        self.loc2seg = {}
        d = {}
        for (y,row) in enumerate(reversed(rows)):
            for (x,c) in enumerate(row):
                if c == '@':
                    self.start = (x,y)
                elif c == '*':
                    self.goal = (x,y)
                elif not c.isalnum():
                    pass
                else:
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

    def getseg(self, x, y):
        return self.loc2seg.get((x,y))

    def show(self):
        for y in xrange(self.height, -1, -1):
            row = []
            for x in xrange(0, self.width):
                if self.start == (x,y):
                    row.append('@')
                elif self.goal == (x,y):
                    row.append('*')
                else:
                    i = self.getseg(x, y)
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
                    i = self.board.getseg(x, y)
                    if i is None:
                        row.append('.')
                    else:
                        row.append(str(self.depths[i]))
            print ' '+''.join(row)
        return

    def getdepth(self, x, y):
        if y < 0:
            return INF
        i = self.board.getseg(x, y)
        if i is None:
            return 0
        else:
            return self.depths[i]

    def getsegs(self, loc):
        (x,y) = loc
        z = self.getdepth(x, y)
        # platform block.
        sp = self.board.getseg(x, y-1)
        zp = self.getdepth(x, y-1)
        assert z < zp, (x,y)
        r = []
        # front block.
        sf = self.board.getseg(x, y)
        if sf is not None and sf != sp and z < zp-1:
            if 0 < y:
                r.append((sf, 0, zp-1))
            else:
                r.append((sf, 0, None))
        # side block. (left)
        sl = self.board.getseg(x-1, y)
        zl = self.getdepth(x-1, y)
        if sl is not None and sl != sp and z < zl:
            r.append((sl, z+1, None))
        # side block. (right)
        sr = self.board.getseg(x+1, y)
        zr = self.getdepth(x+1, y)
        if sr is not None and sr != sp and z < zr:
            r.append((sr, z+1, None))
        return r

    def getlocs(self, loc):
        locs = set()
        def expand(x0, y0):
            if x0 < 0 or self.board.width <= x0: return
            p = (x0,y0)
            if p in locs: return
            locs.add(p)
            z0 = self.getdepth(x0, y0)
            zp = self.getdepth(x0, y0-1)
            assert z0 < zp, (x0,y0)
            # check jumping.
            if self.getdepth(x0, y0+1) < zp:
                for dx in (-1,0,+1):
                    x1 = x0+dx
                    z = self.getdepth(x1, y0+1)
                    if zp <= z: continue
                    if max(z,z0-1) < self.getdepth(x1, y0):
                        #print ' jump1', (x0,y0), (x1, y0+1)
                        expand(x1, y0+1)
                        continue
            for dx in (-1,+1):
                x1 = x0+dx
                if self.getdepth(x1, y0+1) < zp:
                    x2 = x1+dx
                    z = self.getdepth(x2, y0+1)
                    if zp <= z: continue
                    if max(z,z0) < self.getdepth(x2, y0):
                        #print ' jump2', (x0,y0), (x2, y0+1)
                        expand(x2, y0+1)
                        continue
                    z = self.getdepth(x2, y0)
                    if zp <= z: continue
                    if max(z,z0) < self.getdepth(x2, y0-1):
                        #print ' jump3', (x0,y0), (x2, y0)
                        expand(x2, y0)
            # check walking/falling.
            for dx in (-1,+1):
                x1 = x0+dx
                z = self.getdepth(x1, y0)
                if zp <= z: continue
                for y1 in xrange(y0, -1, -1):
                    if max(z,z0) < self.getdepth(x1, y1-1):
                        #print ' walk/fall', (x0,y0), (x1, y1)
                        expand(x1, y1)
                        break
            # check falling.
            for dx in (-1,0,+1):
                x1 = x0+dx
                z = zp
                for y1 in xrange(y0, -1, -1):
                    z1 = self.getdepth(x1, y1-1)
                    if z < z1:
                        #print ' fall', (x0,y0), (x1, y1), z,z1
                        expand(x1, y1)
                        break
                    z = max(z,z1)
        (x,y) = loc
        expand(x, y)
        return locs

def solve_pushmo(board, verbose=True, max_depth=3):
    depths = tuple( 0 for _ in board.segments )
    queue = []
    queue.append((0, None, depths, board.start))
    states = {}
    solution = None
    while queue:
        queue.sort(key=lambda e:e[0])
        prev = queue.pop(0)
        (n, _, depths, loc0) = prev
        n += 1
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
        if verbose:
            print '-- Move %d --' % n
            config.show(loc0)
        newlocs = config.getlocs(loc0)
        if verbose:
            print ' Possible locations:', sorted(newlocs)
            print
        if board.goal in newlocs:
            solution = (n, prev, depths, board.goal)
            break
        locsets.append(newlocs)
        for loc in newlocs:
            for (i,z0,z1) in config.getsegs(loc):
                if z1 is None:
                    z1 = max_depth
                for z in xrange(z0, z1+1):
                    d = depths[:i] + (z,) + depths[i+1:]
                    if d in states: continue
                    queue.append((n, prev, d, loc))
    r = []
    while solution is not None:
        (n, prev, depths, loc) = solution
        r.append((n,depths,loc))
        solution = prev
    r.reverse()
    return r

def main(argv):
    import fileinput
    board = Board([ line.strip() for line in fileinput.input() ])
    print '-- Initial state --'
    board.show()
    print
    steps = solve_pushmo(board)
    if not steps:
        print 'Unsolvable.'
    else:
        for (i,depths,loc) in steps:
            config = Config(board, depths)
            print '-- Move %d --' % i
            config.show(loc)
            print
        print 'Solved in %d steps.' % len(steps)
    return 0

if __name__ == '__main__': sys.exit(main(sys.argv))
