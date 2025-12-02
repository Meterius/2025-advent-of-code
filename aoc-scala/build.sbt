ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "3.3.7"

lazy val root = project.in(file("."))
  .settings(
    scalaVersion := "3.3.7",
    libraryDependencies += "org.scala-lang" %% "toolkit" % "0.7.0",
    libraryDependencies += "com.lihaoyi" %% "os-lib" % "0.11.3"
  )