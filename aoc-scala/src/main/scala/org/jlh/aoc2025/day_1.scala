package org.jlh.aoc2025

private def parseRotation(rotStr: String): Int = {
  val dir = rotStr(0) match
    case 'L' => -1
    case 'R' => 1
    case _ => throw new Error(s"Invalid rotation: $rotStr")

  dir * rotStr.substring(1).toInt
}

def solveDay1Part1(data: Iterator[String]): Int =
  val maxPos = 100

  var pos = 50
  var counter = 0

  for rotStr <- data do {
    pos = scala.math.floorMod(pos + parseRotation(rotStr), maxPos)
    if pos == 0 then
      counter += 1
  }

  counter

def solveDay1Part2(data: Iterator[String]): Int =
  val maxPos = 100

  var pos = 50
  var counter = 0

  for rotStr <- data do {
    val rot = parseRotation(rotStr)
    val rotK = rot / maxPos
    val rotR = rot % maxPos

    counter += rotK.abs

    if pos != 0 then
      counter += scala.math.floorDiv(pos - 1 + rotR, maxPos - 1).abs

    pos = scala.math.floorMod(pos + rotR, maxPos)
  }

  counter