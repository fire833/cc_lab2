#!/usr/bin/env Rscript

library(ggplot2)
library(dplyr)

data <- read.csv("results.csv")

# data_cleaned <- data %>% group_by(data$pattern_name) %>% 
#   filter(!(abs((data$compute_time - mean(data$compute_time)) / sd(data$compute_time)) > 3 * sd(data$compute_time)))

png(sprintf("images/compute_v_pattern_overall.png"))
with(data, boxplot(data$compute_time ~ data$template_name, main = "Overall Compute Time Versus Template Type/Version", xlab = "Template", ylab = "Compute Time (in nanoseconds)"))

data_base <- data %>% filter(template_name == "base2")
data_cuda <- data %>% filter(template_name == "cuda2")
data_simd <- data %>% filter(template_name == "simd2")

data_10000 <- data %>% filter(arg_count == 10000)
data_5000 <- data %>% filter(arg_count == 5000)
data_1000 <- data %>% filter(arg_count == 1000)
data_250 <- data %>% filter(arg_count == 250)
data_50 <- data %>% filter(arg_count == 50)
data_10 <- data %>% filter(arg_count == 10)

png(sprintf("images/compute_v_pattern_10000.png"))
with(data_10000, boxplot(compute_time ~ template_name, main = "10000 Argument Compute Time Versus Template Type/Version", xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
png(sprintf("images/compute_v_pattern_5000.png"))
with(data_5000, boxplot(compute_time ~ template_name, main = "5000 Compute Versus Template Type/Version", xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
png(sprintf("images/compute_v_pattern_1000.png"))
with(data_1000, boxplot(compute_time ~ template_name, main = "1000 Compute Versus Template Type/Version", xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
png(sprintf("images/compute_v_pattern_250.png"))
with(data_250, boxplot(compute_time ~ template_name, main = "250 Compute Versus Template Type/Version", xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
png(sprintf("images/compute_v_pattern_50.png"))
with(data_50, boxplot(compute_time ~ template_name, main = "50 Compute Versus Template Type/Version", xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
png(sprintf("images/compute_v_pattern_10.png"))
with(data_10, boxplot(compute_time ~ template_name, main = "10 Compute Versus Template Type/Version", xlab = "Template", ylab = "Compute Time (in nanoseconds)"))

for (index in 0:4) {
  print(index)
  data_10000 <- data %>% filter(arg_count == 10000 & pattern_number == index)
  print(length(data_10000$compute_time))
  data_5000 <- data %>% filter(arg_count == 5000 & pattern_number == index)
  print(length(data_5000$compute_time))
  data_1000 <- data %>% filter(arg_count == 1000 & pattern_number == index)
  print(length(data_1000$compute_time))
  data_250 <- data %>% filter(arg_count == 250 & pattern_number == index)
  print(length(data_250$compute_time))
  data_50 <- data %>% filter(arg_count == 50 & pattern_number == index)
  print(length(data_50$compute_time))
  data_10 <- data %>% filter(arg_count == 10 & pattern_number == index)
  print(length(data_10$compute_time))

  png(sprintf("images/compute_v_pattern_10000_%d.png", index))
  with(data_10000, boxplot(compute_time ~ template_name, main = sprintf("10000 Compute Versus Template Type/Version (Pattern %d)", index), xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
  png(sprintf("images/compute_v_pattern_5000_%d.png", index))
  with(data_5000, boxplot(compute_time ~ template_name, main = sprintf("5000 Compute Versus Template Type/Version (Pattern %d)", index), xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
  png(sprintf("images/compute_v_pattern_1000_%d.png", index))
  with(data_1000, boxplot(compute_time ~ template_name, main = sprintf("1000 Compute Versus Template Type/Version (Pattern %d)", index), xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
  png(sprintf("images/compute_v_pattern_250_%d.png", index))
  with(data_250, boxplot(compute_time ~ template_name, main = sprintf("250 Compute Versus Template Type/Version (Pattern %d)", index), xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
  png(sprintf("images/compute_v_pattern_50_%d.png", index))
  with(data_50, boxplot(compute_time ~ template_name, main = sprintf("50 Compute Versus Template Type/Version (Pattern %d)", index), xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
  png(sprintf("images/compute_v_pattern_10_%d.png", index))
  with(data_10, boxplot(compute_time ~ template_name, main = sprintf("10 Compute Versus Template Type/Version (Pattern %d)", index), xlab = "Template", ylab = "Compute Time (in nanoseconds)"))
}
