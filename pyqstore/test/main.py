from pyqstore import QStoreMemory
import rdflib
from rdflib import Graph, URIRef, Literal


def main():
    m = QStoreMemory()
    print(m)
    g = Graph(m, "mygraph")
    g.add((URIRef("http://example.com/mytest1"), Literal("http://example.com/is", lang="en"), URIRef("http://example.com#great")))
    t = g.triples((None, None, None))
    print(t)
    for t1 in t:
        print(t1)
        s,p,o = t1
        print(str(s), str(p._value), str(p._language), str(o))


if __name__ == "__main__":
    main()

