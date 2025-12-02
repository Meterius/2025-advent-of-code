package org.jlh.aoc2025

@main
def run(args: String*): Unit = {
  val day = args.lift(0).map(_.toInt)

  val solvers = Array(
    (solve_day1_part1, solve_day1_part2)
  );

  for ((solve_part1, solve_part2), idx) <- solvers.view.zipWithIndex do {
    val day_idx = idx + 1
    if day.fold(true)(day_idx == _) then
      println(s"Day $day_idx:")
      
      val data = scala.io.Source.fromResource(s"day_${day_idx}_data.txt").getLines()
      val part1 = solve_part1(data)
      println(s"\tPart 1: $part1")
      val part2 = solve_part2(data)
      println(s"\tPart 2: $part2")
  }
}