#!/bin/bash -f
# ****************************************************************************
# Vivado (TM) v2019.2 (64-bit)
#
# Filename    : elaborate.sh
# Simulator   : Xilinx Vivado Simulator
# Description : Script for elaborating the compiled design
#
# Generated by Vivado on Mon Apr 13 18:51:52 CEST 2020
# SW Build 2708876 on Wed Nov  6 21:39:14 MST 2019
#
# Copyright 1986-2019 Xilinx, Inc. All Rights Reserved.
#
# usage: elaborate.sh
#
# ****************************************************************************
set -Eeuo pipefail
echo "xelab -wto 824e2c52fee14aeeb0c23e19c6ea2648 --incr --debug typical --relax --mt 8 -L xil_defaultlib -L secureip --snapshot sram_tb_behav xil_defaultlib.sram_tb -log elaborate.log"
xelab -wto 824e2c52fee14aeeb0c23e19c6ea2648 --incr --debug typical --relax --mt 8 -L xil_defaultlib -L secureip --snapshot sram_tb_behav xil_defaultlib.sram_tb -log elaborate.log
