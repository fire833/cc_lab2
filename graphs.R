#!/usr/bin/env Rscript

library(ggplot2)
library(dplyr)

data <- read.csv("results.csv")
cleaned_data <- lapply(data, function(col) {
  z <- ktrutils::get_z(col)
  return(col[abs(z) <= 3])
})

base2_10000 <- cleaned_data$base2_10000_0_prog.c %>% mutate(category = "base2_10000")
simd2_10000 <- cleaned_data$simd2_10000_0_prog.c %>% mutate(category = "simd2_10000")
cuda2_10000 <- cleaned_data$cuda2_10000_0_prog.c %>% mutate(category = "cuda2_10000")
data_10000 <- append(data_10000, base2_10000, simd2_10000, cuda2_10000)
png(format("images/pattern_len_10000_%d.png", index))

layout(matrix(1:3, nr = 1, nc = 3, byrow = TRUE))
with(cleaned_data, boxplot(base2_10000_0_prog.c, main = "", col = "Blue"))
with(cleaned_data, boxplot(simd2_10000_0_prog.c, main = "", col = "Red"))
with(cleaned_data, boxplot(cuda2_10000_0_prog.cu, main = "", col = "Green"))
