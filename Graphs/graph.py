import random, queue
class graph:
    def __init__(self):
       self.V=set()
       self.E=dict()

       return
    def addVertex(self,v):
       if not v in self.V:
           self.V.add(v)
           self.E[v] = set()
       return
   
    def addEdge(self,e):
        u,v = e
        self.addVertex(u)
        self.addVertex(v)
        if not u in self.E[v]:
            self.E[v].add(u)
        if not v in self.E[u]:
            self.E[u].add(v)
        return
    
    def edgeExists(self,u,v):
        if u in self.E[v]:
            return True
        return False
    
    def removeEdge(self, e):
        u,v = e
        self.addVertex(u)
        self.addVertex(v)
        if u in self.E[v]:
            self.E[v].remove(u)
        if v in self.E[u]:
            self.E[u].remove(v)
        return
    
    def removeAllEdges(self):
        for v in self.V:
            self.E[v].clear()
        return
    
    def neighbors(self, v):
        self.addVertex(v)
        return self.E[v]
    
    def makeComplete(self):
       for v in self.V:
           for w in self.V:
               if v < w:
                   e = (v,w)
                   self.addEdge(e)
       return
    def erdos(self, P):
        self.makeComplete()
        toRemove=[]
        for e in self.E:
            if random.random() <= P:
                toRemove.append(e)
        for e in toRemove:
            self.E.remove(e)
        return
    def makeRegular(self,K):
        self.removeAllEdges()
        vlist=list(self.V)
        for i in range(len(vlist)):
            for j in range(i+1, i+K):
                e = (vlist[i],vlist[j%len(vlist)])
                self.addEdge(e)
        return
    def rewire(self, P):
       elist = list(self.E)
       vlist=list(self.V)
       for e in elist:
           if random.random()<=P:
               self.E.remove(e)
               (u,v) = e
               v = random.choice(vlist)
               e = (u,v)
               self.addEdge(e)
       return
    def __str__(self):
       Vertex = ""
       for v in self.V:
           Vertex += str(v)+":"
           
       edges = ""
       for e in self.E:
           edges += str(e)+":"
           
       return Vertex + '\n' + edges
    def toCSV(self):
        edges = ""
        for u,v in self.E:
           edges += f"{u}, {v}\n"
        return edges
   
    def degreeDist(self):
        degree = [0]*len(self.V)
        for (u,v) in self.E:
            degree[u]=degree[u]+1
            degree[v]=degree[v]+1
        dist = [0]*len(self.V)
        for v in degree:
            dist[v]=dist[v]+1
        return dist

    def sp(self,s,t):
        q = queue.Queue()
        used = [False]*len(self.V)
        d =[0]*len(self.V)
        P =[0]*len(self.V)
        
        q.put(s)
        used[s]=True
        P[s]=-1
        while(q.empty()==False):
            v = q.get();
            for u in self.adj[v]:
                if used[u] == False:
                    used[u]=True
                    q.put(u)
                    d[u]=d[v]+1
                    P[u]=v
        return d[t]
    
    def numTriangles(self):
        count = 0 
        for i in self.V:
            for j in self.V:
                for K in self.V:
                    if i<j and j<K:
                        if self.edgeExists(i,j) and\
                        self.edgeExists(i,K) and\
                        self.edgeExists(j,K):
                            count = count+1
        return count
    
    def kstar(self,K):
        count = 0
        for i in self.V:
            if len(self.neighbors(i)) >= K:
                count = count+1
        return count
                        
mygraph = graph()

# for i in range(20):
#     mygraph.addVertex(i)
# mygraph.makeRegular(3)
# mygraph.rewire()
# mygraph.E.clear()
# mygraph.makeComplete()
# print(mygraph)
# print(mygraph.sp())

for i in range(5):
    mygraph.addVertex(i)
mygraph.addEdge((0,1))
mygraph.addEdge((1,2))
mygraph.addEdge((1,3))
mygraph.addEdge((3,4))
print(mygraph.sp(0,0))

       
      

