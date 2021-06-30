import random
import queue
class Graph:
    #constructor
    def __init__(self, V=[],E=[]):
        """
        Initializes the undirected graph object.

        Parameters
        ----------
        V : TYPE, optional
            DESCRIPTION. List of vertices.
        E : TYPE, optional
            DESCRIPTION. List of edges. Each edge is a tuple of two vertices.

        Returns
        -------
        None.

        """
        self.data=dict()
        for v in V:
            self.add_vertex(v)
            
        for e in E:
            self.add_edge(e)
            
        return
    
    def get_vertices(self):
        """
        Returns a list of vertices in this graph.

        Returns
        -------
        TYPE
            DESCRIPTION. Returns a list of vertices in this graph.

        """
        return list(self.data.keys())
    
    def get_edges(self):
        """
        Returns a set of edges in this graph. Each edge is a 2-tuple that contains
        the two vertices on which the edge is incident.

        Returns
        -------
        E : TYPE
            DESCRIPTION.

        """
        E = set()
        for v in self.data.values():
            E.update(v.keys())
        return E
    
    def has_edge(self, x, y):
        """
        Check if there is an edge between two given vertices.

        Parameters
        ----------
        x : TYPE
            DESCRIPTION. A certex.
        y : TYPE
            DESCRIPTION. A vertex.

        Returns
        -------
        bool
            DESCRIPTION. Returns True if there is an edge between vertex x
            and vertex y; False otherwise.

        """
        if y in self.data[x].values():
            return True
        return False
        
    def get_triangles(self):
        """
        Returns the number of distinct triples (a, b, c) such that a, b and c
        are vertices in the graph, and (a,b), (b,c) and (a,c) are edges in the graph.

        Returns
        -------
        TYPE
            DESCRIPTION. Returns the number of distinct triples (a, b, c) such that a, b and c
        are vertices in the graph, and (a,b), (b,c) and (a,c) are edges in the graph.

        """
        count = 0
        V = self.get_vertices()
        for x in V:
            for y in V:
                for z in V:
                    if x==y or y==z or x==z:
                        continue
                    if self.has_edge(x,y) and self.has_edge(y, z) and self.has_edge(x, z):
                        count += 1
        return count//6
    
    def add_vertex(self, v):
        """
        Adds a vertex to the graph.

        Parameters
        ----------
        v : TYPE
            DESCRIPTION. A vertex.

        Returns
        -------
        None.

        """
        self.data[v] = dict()
        return
        
    def add_edge(self, e):
        """
        Adds an edge to the graph.

        Parameters
        ----------
        e : TYPE
            DESCRIPTION. An edge represented as a 2-tuple that contains the
            two endpoints of the edge.

        Returns
        -------
        None.

        """
        x,y=e
        if x not in self.data:
            self.add_vertex(x)
            
        if y not in self.data:
            self.add_vertex(y)
            
        self.data[x][e]=y
        self.data[y][e]=x
        return
    
    def make_regular(self, K):
        V = list(self.data.keys())
        N = len(V)
        for i in range(len(V)):
            for j in range(1, K+1):
                x = V[i]
                y = V[(i+j)%N]
                self.add_edge((x,y))
                y = V[(i-j+N)%N]
        
        return
    
    def rewire(self, P): #create a small world random graph
        V = self.data.keys()
        for v in V:
            edges = list(self.data[v].keys())
            
            choices = list(V) 
            
            for e in edges: #these are vertices to which v does not have an edge
                if self.data[v][e] in choices:
                    choices.remove(self.data[v][e])
            choices.remove(v)
            
            for e in edges:
                if random.random() < P:
                    (x,y) = e
                    del self.data[x][e]
                    del self.data[y][e]
                    y = random.choice(choices)
                    self.add_edge((v,y))
                    choices.remove(y)
        return
    
    def __str__(self):
        out = ""
        for v in self.data.keys():
            for e in self.data[v]:
                out += str(v) + "," + str(self.data[v][e]) + "\n"
                
        return out
    
    def bfs(self, v):
        """
        Use BFS to compute lengths of shortest paths from v to all the vertices
        of the graph.

        Parameters
        ----------
        v : TYPE
            DESCRIPTION.

        Returns
        -------
        TYPE
            DESCRIPTION. A dictionry with all vertices as keys, and the corresponding
            values between the lengths of the shortest paths from v to those respective
            keys.

        """
        F = queue.Queue()
        F.put(v)
        dist = dict()
        dist[v] = 0
        while F.empty() == False:
          x = F.get()
          for y in self.data[x].values():
              if y not in dist:
                  dist[y] = dist[x]+1
                  F.put(y)
        for x in self.get_vertices():
            if x not in dist:
                dist[x] = float("inf")
        return dist
        # total = 0
        # for d in dist.values():
        #     total = total + d
        # return total/len(self.data.keys())          
      
    def avg_short_path(self):
        total = 0
        for v in self.get_vertices():
            dist = self.bfs(v)
            total = total + sum(dist.values())/len(self.get_vertices())
        return total/len(self.get_vertices())
    def degree(self, v):
        total = 0
        for w in self.get_vertices():
            if self.has_edge(v, w):
                total = total + 1
        return total
    def degree_distribution(self):
        distrib = dict()
        n = len(self.get_vertices())
        for i in range(0,n):
            distrib[i]=0
        for v in self.get_vertices():
            distrib[self.degree(v)] = distrib[self.degree(v)] + 1
        return distrib
G = Graph(V = [1,2,3,4])
G.add_edge((1,2))
G.add_edge((1,3))
G.add_edge((1,4))
G.add_edge((2,3))
G.add_edge((2,4))
G.add_edge((3,4))
print(G.degree_distribution())