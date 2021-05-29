class DTree(object):
    def __init__(self, children):
        self.children = children

class DEnt(object):
    def __init__(self, name, subdir):
        self.name = name
        self.subdir = subdir

def paths(dt):
    if not dt.children:
        return ["/"]
    result = []
    for d in dt.children:
        subpaths = paths(d.subdir)
        for sp in subpaths:
            result.append("/" + d.name + sp)
    return result

dt = DTree([
    DEnt("a", DTree([
        DEnt("b", DTree([
            DEnt("d", DTree([]))
        ])),
        DEnt("c", DTree([])),
    ]))
])

print(sorted(paths(dt)))
