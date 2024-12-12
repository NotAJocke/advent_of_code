defmodule GardenGroups do
  def input do
    """
    AAAA
    BBCD
    BBCC
    EEEC
    """
  end

  def input2 do
    """
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE
    """
  end

  def get_input(false), do: input()
  def get_input(true), do: File.read!("inputs/garden_groups.txt")

  def parse(string) do
    string
    |> String.trim()
    |> String.split("\n", trim: true)
    |> Enum.map(&String.graphemes/1)
  end

  def matrix_to_map(matrix) do
    matrix
    |> Enum.with_index()
    |> Enum.reduce(Map.new(), fn {row, row_idx}, acc ->
      row
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {cell, cell_idx}, acc ->
        Map.put(acc, {cell_idx, row_idx}, cell)
      end)
    end)
  end

  def walk_garden_perim(map) do
    map
    |> Map.keys()
    |> Enum.reduce({MapSet.new(), Map.new(), Map.new()}, fn {x, y}, {visited, regions, sides} ->
      if MapSet.member?(visited, {x, y}) do
        {visited, regions, sides}
      else
        {new_visited, {area, perimeter}, delimiters} =
          walk_garden_perim(map, {x, y}, MapSet.new())

        updated_visited = MapSet.union(visited, new_visited)

        updated_regions =
          Map.update(regions, Map.get(map, {x, y}), [{area, perimeter}], fn existing ->
            [{area, perimeter} | existing]
          end)

        updated_sides =
          Map.update(sides, Map.get(map, {x, y}), delimiters, fn existing ->
            [delimiters | existing]
          end)

        {updated_visited, updated_regions, updated_sides}
      end
    end)
  end

  def walk_garden_perim(map, {x, y}, visited) do
    plant_type = Map.get(map, {x, y})
    walk_garden_perim(map, [{x, y}], visited, [], plant_type, 0, 0)
  end

  def walk_garden_perim(_map, [], visited, sides, _plant_type, area, perimeter),
    do: {visited, {area, perimeter}, sides}

  def walk_garden_perim(map, [hd | tl], visited, sides, plant_type, area, perimeter) do
    if(MapSet.member?(visited, hd)) do
      walk_garden_perim(map, tl, visited, sides, plant_type, area, perimeter)
    else
      # mark visited
      visited = MapSet.put(visited, hd)
      {x, y} = hd

      neighbors = [
        # up
        {x, y - 1},
        # down
        {x, y + 1},
        # left
        {x - 1, y},
        # right
        {x + 1, y}
      ]

      {valid_neighbors, local_perimeter, delim} =
        Enum.reduce(neighbors, {[], 0, []}, fn neighbor, {valid, peri, delim} ->
          case Map.get(map, neighbor) do
            ^plant_type -> {[neighbor | valid], peri, delim}
            # Out of bounds
            nil -> {valid, peri + 1, [neighbor | delim]}
            # Different plant type
            _ -> {valid, peri + 1, [neighbor | delim]}
          end
        end)

      walk_garden_perim(
        map,
        tl ++ valid_neighbors,
        visited,
        [delim | sides],
        plant_type,
        area + 1,
        perimeter + local_perimeter
      )
    end
  end

  def fence_price(regions) do
    Enum.reduce(regions, Map.new(), fn {plant, regions_list}, acc ->
      Enum.reduce(regions_list, acc, fn {area, perimeter}, acc_inner ->
        Map.update(acc_inner, plant, area * perimeter, fn price ->
          area * perimeter + price
        end)
      end)
    end)
  end

  def keep_uniques_intersections({visited, regions, intersections}) do
    new_intersections =
      intersections
      |> Enum.reduce(Map.new(), fn {k, v}, acc ->
        uniques =
          v
          |> List.flatten()
          |> Enum.uniq()
          |> Enum.sort()

        Map.put(acc, k, uniques)
      end)

    {visited, regions, new_intersections}
  end

  def initial_direction({a, b}, {x, y}) do
    # up
    cond do
      x - a == 1 and b == y -> :right
      a - x == 1 and b == y -> :left
      a == x and y - b == 1 -> :down
      a == x and b - y == 1 -> :up
      true -> :error
    end
  end

  def count_sides(path) do
    [start | tl] = path
  end

  def run1(use_puzzle \\ false) do
    get_input(use_puzzle)
    |> parse()
    |> matrix_to_map()
    |> walk_garden_perim()
    |> (fn {_visited, regions, _sides} -> fence_price(regions) end).()
    |> Enum.reduce(0, fn {_plant, price}, acc -> acc + price end)
  end

  def run2(use_puzzle \\ false) do
    get_input(use_puzzle)
    |> parse()
    |> matrix_to_map()
    |> walk_garden_perim()
    |> keep_uniques_intersections()
  end

  def temp do
    x = [
      {-1, 0},
      {0, -1},
      {0, 1},
      {1, -1},
      {1, 1},
      {2, -1},
      {2, 1},
      {3, -1},
      {3, 1},
      {4, 0}
    ]
  end
end
