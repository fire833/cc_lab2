#!/usr/bin/env Rscript

library(ggplot2)
library(dplyr)

data <- read.csv("results.csv")
data$pattern_number <- as.factor(data$pattern_number)
data$arg_count <- as.factor(data$arg_count)

data_base <- data %>% filter(template_name == "base2")
data_cuda <- data %>% filter(template_name == "cuda2")
data_simd <- data %>% filter(template_name == "simd2")

data_10000 <- data %>% filter(arg_count == 10000)
data_5000 <- data %>% filter(arg_count == 5000)
data_1000 <- data %>% filter(arg_count == 1000)
data_250 <- data %>% filter(arg_count == 250)
data_50 <- data %>% filter(arg_count == 50)
data_10 <- data %>% filter(arg_count == 10)

png(sprintf("images/compute_v_template_10000.png"))
ggplot(data_10000, aes(x = compute_time, t = template_name, fill = template_name)) +
  geom_boxplot() +
  ggtitle("Compute vs. Template 10000 Arguments") +
  xlim(0, 12000)

png(sprintf("images/compute_v_template_5000.png"))
ggplot(data_5000, aes(x = compute_time, t = template_name, fill = template_name)) +
  geom_boxplot() +
  ggtitle("Compute vs. Template 5000 Arguments") +
  xlim(0, 12000)

png(sprintf("images/compute_v_template_1000.png"))
ggplot(data_1000, aes(x = compute_time, t = template_name, fill = template_name)) +
  geom_boxplot() +
  ggtitle("Compute vs. Template 1000 Arguments") +
  xlim(0, 12000)

png(sprintf("images/compute_v_template_250.png"))
ggplot(data_250, aes(x = compute_time, t = template_name, fill = template_name)) +
  geom_boxplot() +
  ggtitle("Compute vs. Template 250 Arguments") +
  xlim(0, 12000)

png(sprintf("images/compute_v_template_50.png"))
ggplot(data_50, aes(x = compute_time, t = template_name, fill = template_name)) +
  geom_boxplot() +
  ggtitle("Compute vs. Template 50 Arguments") +
  xlim(0, 12000)

png(sprintf("images/compute_v_template_10.png"))
ggplot(data_10, aes(x = compute_time, t = template_name, fill = template_name)) +
  geom_boxplot() +
  ggtitle("Compute vs. Template 10 Arguments") +
  xlim(0, 12000)

png(sprintf("images/compute_v_template_all.png"))
ggplot(data, aes(x = compute_time, y = template_name, z = arg_count, fill = arg_count)) +
  geom_boxplot() +
  ggtitle("Compute vs. Template by Argument Count") +
  xlim(0, 12000)
