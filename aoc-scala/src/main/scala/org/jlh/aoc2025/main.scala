package org.jlh.aoc2025

@main
def run(args: String*): Unit = {
  val day = args.lift(0).map(_.toInt)

  val solvers = Array(
    (solveDay1Part1, solveDay1Part2)
  );

  for ((solvePart1, solvePart2), idx) <- solvers.view.zipWithIndex do {
    val day_idx = idx + 1
    if day.fold(true)(day_idx == _) then
      println(s"Day $day_idx:")
      
      val data = scala.io.Source.fromResource(s"day_${day_idx}_data.txt").getLines()
      val part1 = solvePart1(data)
      println(s"\tPart 1: $part1")

      val data2 = scala.io.Source.fromResource(s"day_${day_idx}_data.txt").getLines()
      val part2 = solvePart2(data2)
      println(s"\tPart 2: $part2")
  }
}