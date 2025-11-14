from collections import defaultdict, deque

class FordFulkerson:
    def __init__(self, vertices):
        self.graph = defaultdict(dict)
        self.vertices = vertices
    
    def add_edge(self, u, v, capacity):
        """Добавление ребра с пропускной способностью"""
        self.graph[u][v] = capacity
        if v not in self.graph:
            self.graph[v] = {}
    
    def bfs(self, source, sink, parent):
        """Поиск пути от истока к стоку с помощью BFS"""
        visited = {source}
        queue = deque([source])
        
        while queue:
            u = queue.popleft()
            
            for v in self.graph[u]:
                if v not in visited and self.graph[u][v] > 0:
                    visited.add(v)
                    queue.append(v)
                    parent[v] = u
                    if v == sink:
                        return True
        return False
    
    def ford_fulkerson(self, source, sink):
        """Алгоритм Форда-Фалкерсона для поиска максимального потока"""
        parent = {}
        max_flow = 0
        paths = []  # Для хранения увеличивающих путей
        
        # Создаем копию графа для остаточной сети
        residual_graph = defaultdict(dict)
        for u in self.graph:
            for v in self.graph[u]:
                residual_graph[u][v] = self.graph[u][v]
                if u not in residual_graph[v]:
                    residual_graph[v][u] = 0
        
        self.graph = residual_graph
        
        # Поиск увеличивающих путей
        iteration = 0
        while self.bfs(source, sink, parent):
            iteration += 1
            # Находим минимальную пропускную способность на пути
            path_flow = float('inf')
            s = sink
            path = []
            
            while s != source:
                path.append(s)
                path_flow = min(path_flow, self.graph[parent[s]][s])
                s = parent[s]
            path.append(source)
            path.reverse()
            
            paths.append((iteration, path, path_flow))
            
            # Обновляем остаточные пропускные способности
            v = sink
            while v != source:
                u = parent[v]
                self.graph[u][v] -= path_flow
                self.graph[v][u] += path_flow
                v = parent[v]
            
            max_flow += path_flow
            parent = {}
        
        return max_flow, paths

# Создание графа из изображения
vertices = ['S', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 't']
ff = FordFulkerson(vertices)

# Добавление рёбер согласно графу на изображении
ff.add_edge('S', 'A', 5)
ff.add_edge('S', 'B', 4)
ff.add_edge('A', 'D', 1)
ff.add_edge('A', 'C', 3)
ff.add_edge('C', 'D', 2)
ff.add_edge('C', 'B', 1)
ff.add_edge('C', 'E', 2)
ff.add_edge('C', 'F', 3)
ff.add_edge('B', 'F', 2)
ff.add_edge('D', 'G', 3)
ff.add_edge('E', 'G', 2)
ff.add_edge('E', 't', 2)
ff.add_edge('G', 't', 5)

# Поиск максимального потока
max_flow, paths = ff.ford_fulkerson('S', 't')

print("=" * 60)
print("АЛГОРИТМ ФОРДА-ФАЛКЕРСОНА")
print("=" * 60)
print(f"\nИсток: S")
print(f"Сток: t")
print(f"\nНайденные увеличивающие пути:\n")

for iteration, path, flow in paths:
    path_str = " → ".join(path)
    print(f"Итерация {iteration}: {path_str}")
    print(f"  Поток на пути: {flow}")
    print()

print("=" * 60)
print(f"МАКСИМАЛЬНЫЙ ПОТОК: {max_flow}")
print("=" * 60)
