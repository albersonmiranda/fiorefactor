# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_fiorefactor_wrappers", use_symbols = TRUE, package_name = "fiorefactor")

#' @usage NULL
#' @useDynLib fiorefactor, .registration = TRUE
NULL

#' Rust class for input-output matrix
#' @description
#' This class represents an input-output matrix, which is a representation of the transactions between different sectors of an economy.
#' It contains methods to compute the technical coefficients matrix and other related operations.
#' @usage
#' Iom$new(
#'   name,
#'   intermediate_transactions,
#'   total_production
#' )
#' @format NULL
#' @param name (`character`)\cr
#' A string representing the name of the input-output matrix.
#' @param intermediate_transactions (`matrix`)\cr
#' A matrix of intermediate transactions.
#' @param total_production (`matrix`)\cr A vector of total production.
#' @return A new instance of the `Iom` class.
#' 
#' @details
#' This paragraph of details is on struct-level.
#' 
#' @export
#' @details
#' But hey! This impl-block adds stuff that requires clarification so this paragraph
#' is on impl-level docs but it's appended to struct docs! Isn't that cool?
#' @section Methods:
#'\subsection{Method `new`}{
#'Instantiate a new Iom object
#' \subsection{Arguments}{
#'\describe{
#'\item{`name`}{(`character`) A string representing the name of the input-output matrix.}
#'\item{`intermediate_transactions`}{(`matrix`) A matrix of intermediate transactions.}
#'\item{`total_production`}{(`character`) A vector of total production.}
#'}}
#' \subsection{details}{
#'This function creates a new instance of the Iom class.
#'}
#' \subsection{return}{
#'A new instance of the Iom class.
#'}
#' \subsection{examples}{
#' \preformatted{
#'Iom$new(
#'name = "example",
#'intermediate_transactions = c(1, 2, 3, 4),
#'total_production = c(5, 6)
#')
#'}
#'}
#'}
#'
#'\subsection{Method `intermediate_transactions`}{
#'Getter for intermediate_transactions matrix.
#'}
#'
#'\subsection{Method `total_production`}{
#'Getter for total_production matrix.
#'}
#'
#'\subsection{Method `technical_coefficients_matrix`}{
#'Getter for technical_coefficients_matrix.
#'}
#'
#'
#' @section Methods:
#'\subsection{Method `compute_technical_coefficients`}{
#'Compute the technical coefficients matrix and populate the `technical_coefficients_matrix` field.
#' \subsection{usage}{
#' \preformatted{
#'Iom$compute_technical_coefficients()
#'}
#'}
#' \subsection{details}{
#'It computes the technical coefficientex matrix, a nxn matrix, known as `A` matrix, which is the column-wise ratio of intermediate transactions to total production.
#'}
#' \subsection{return}{
#'Self (invisibly)
#'}
#' \subsection{examples}{
#' \preformatted{
#'iom <- Iom$new(
#'name = "example",
#'intermediate_transactions = c(1, 2, 3, 4),
#'total_production = c(5, 6)
#')
#'iom$compute_technical_coefficients()
#'iom$technical_coefficients_matrix
#'}
#'}
#'}
#'
Iom <- new.env(parent = emptyenv())

Iom$new <- function(name, intermediate_transactions, total_production) .Call(wrap__Iom__new, name, intermediate_transactions, total_production)

Iom$print <- function() invisible(.Call(wrap__Iom__print, self))

Iom$name <- function() .Call(wrap__Iom__name, self)

Iom$intermediate_transactions <- function() .Call(wrap__Iom__intermediate_transactions, self)

Iom$total_production <- function() .Call(wrap__Iom__total_production, self)

Iom$technical_coefficients_matrix <- function() .Call(wrap__Iom__technical_coefficients_matrix, self)

Iom$compute_technical_coefficients <- function() invisible(.Call(wrap__Iom__compute_technical_coefficients, self))

#' @rdname Iom
#' @usage NULL
#' @export
`$.Iom` <- function (self, name) { func <- Iom[[name]]; environment(func) <- environment(); func }

#' @export
`[[.Iom` <- `$.Iom`

#' A test class to demonstrate the use of extendr.
#' @description
#' This is a test struct to check if it's working with multiple structs.
#' @usage
#' Test$new(name, value)
#' @format NULL
#' @param name (`character`)\cr
#' A string representing the name of the test.
#' @param value (`numeric`)\cr
#' A numeric value representing the value of the test.
#' @return A new instance of the `Test` class.
#'
#' @section Methods:
#'\subsection{Method `new`}{
#'Instantiate a new Test object
#' \subsection{Arguments}{
#'\describe{
#'\item{`name`}{(`character`)\cr A string representing the name of the test.}
#'\item{`value`}{(`numeric`)\cr A numeric value representing the value of the test.}
#'}}
#' \subsection{details}{
#'This function creates a new instance of the Test class.
#'}
#' \subsection{return}{
#'A new instance of the Test class.
#'}
#' \subsection{examples}{
#' \preformatted{
#'Test$new(name = "example", value = 42)
#'}
#'}
#'}
#'
#'\subsection{Method `get_name`}{
#' \subsection{details}{
#'This function returns the name of the test.
#'}
#' \subsection{return}{
#'The name of the test.
#'}
#' \subsection{examples}{
#' \preformatted{
#'test <- Test$new(name = "example", value = 42)
#'test$get_name()
#'}
#'}
#'}
#'
#'\subsection{Method `get_value`}{
#' \subsection{details}{
#'This function returns the value of the test.
#'}
#' \subsection{return}{
#'The value of the test.
#'}
#' \subsection{examples}{
#' \preformatted{
#'test <- Test$new(name = "example", value = 42)
#'test$get_value()
#'}
#'}
#'}
#'
Test <- new.env(parent = emptyenv())

Test$new <- function(name, value) .Call(wrap__Test__new, name, value)

Test$get_name <- function() .Call(wrap__Test__get_name, self)

Test$get_value <- function() .Call(wrap__Test__get_value, self)

#' @export
`$.Test` <- function (self, name) { func <- Test[[name]]; environment(func) <- environment(); func }

#' @export
`[[.Test` <- `$.Test`


# nolint end
