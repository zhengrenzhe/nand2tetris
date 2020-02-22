HardwareSimulator = $(abspath ./tools/HardwareSimulator.sh)
CPUEmulator = $(abspath ./tools/CPUEmulator.sh)
Assembler = $(abspath ./tools/Assembler.sh)
VMEmulator = $(abspath ./tools/VMEmulator.sh)

PROJ_1 = $(abspath ./projects/01)
PROJ_2 = $(abspath ./projects/02)
PROJ_3 = $(abspath ./projects/03)
PROJ_4 = $(abspath ./projects/04)
PROJ_5 = $(abspath ./projects/05)
PROJ_7 = $(abspath ./projects/07)
PROJ_8 = $(abspath ./projects/08)

Blue=\033[1;34m
NC=\033[0m

define call_tool
	@for file in $(2); do \
		$(1) $${file}; \
	done
endef

all: project1 project2 project3 project4 project5 project7 project8 clean

project1: $(PROJ_1)/*.tst
	@echo "${Blue}Testing $@... ${NC}"
	$(call call_tool, $(HardwareSimulator), $^)

project2: $(PROJ_2)/*.tst
	@echo "${Blue}Testing $@... ${NC}"
	$(call call_tool, $(HardwareSimulator), $^)

project3: $(PROJ_3)/**/*.tst
	@echo "${Blue}Testing $@... ${NC}"
	$(call call_tool, $(HardwareSimulator), $^)

project4:
	@echo "${Blue}Testing $@... ${NC}"
	$(call call_tool, $(Assembler), $(PROJ_4)/fill/Fill.asm)
	$(call call_tool, $(CPUEmulator), $(PROJ_4)/fill/FillAutomatic.tst)
	$(call call_tool, $(Assembler), $(PROJ_4)/mult/Mult.asm)
	$(call call_tool, $(CPUEmulator), $(PROJ_4)/mult/Mult.tst)

project5: $(PROJ_5)/CPU-external.tst $(PROJ_5)/CPU.tst $(PROJ_5)/ComputerAdd-external.tst $(PROJ_5)/ComputerAdd.tst $(PROJ_5)/ComputerMax-external.tst $(PROJ_5)/ComputerMax.tst $(PROJ_5)/ComputerRect-external.tst $(PROJ_5)/ComputerRect.tst
	@echo "${Blue}Testing $@... ${NC}"
	$(call call_tool, $(HardwareSimulator), $^)

project7:
	@echo "${Blue}Testing $@... ${NC}"
	@cd compiler ;\
	for file in $(PROJ_7)/**/**/*.vm; do \
		cargo -q run -p vm-compiler $${file} ;\
	done ;\
	for file in $(PROJ_7)/MemoryAccess/BasicTest/BasicTest.tst $(PROJ_7)/MemoryAccess/PointerTest/PointerTest.tst $(PROJ_7)/MemoryAccess/StaticTest/StaticTest.tst $(PROJ_7)/StackArithmetic/StackTest/StackTest.tst $(PROJ_7)/StackArithmetic/SimpleAdd/SimpleAdd.tst; do \
		$(CPUEmulator) $${file} ;\
	done

project8:
	@echo "${Blue}Testing $@... ${NC}"
	@cd compiler ;\
	cargo -q run -p vm-compiler $(PROJ_8)/ProgramFlow/BasicLoop/BasicLoop.vm ;\
	cargo -q run -p vm-compiler $(PROJ_8)/ProgramFlow/FibonacciSeries/FibonacciSeries.vm;\
	cargo -q run -p vm-compiler $(PROJ_8)/FunctionCalls/SimpleFunction/SimpleFunction.vm;\
	cargo -q run -p vm-compiler $(PROJ_8)/FunctionCalls/NestedCall;\
	cargo -q run -p vm-compiler $(PROJ_8)/FunctionCalls/FibonacciElement;\
	for file in $(PROJ_8)/ProgramFlow/BasicLoop/BasicLoop.tst $(PROJ_8)/ProgramFlow/FibonacciSeries/FibonacciSeries.tst $(PROJ_8)/FunctionCalls/SimpleFunction/SimpleFunction.tst $(PROJ_8)/FunctionCalls/NestedCall/NestedCall.tst $(PROJ_8)/FunctionCalls/FibonacciElement/FibonacciElement.tst ; do \
		$(CPUEmulator) $${file} ;\
	done

clean:
	@rm $(PROJ_8)/**/**/*.out
	@rm $(PROJ_8)/**/**/*.asm
	@rm $(PROJ_7)/**/**/*.out
	@rm $(PROJ_7)/**/**/*.asm
	@rm $(PROJ_5)/*.out
	@rm $(PROJ_4)/**/*.out
	@rm $(PROJ_4)/**/*.hack
	@rm $(PROJ_3)/**/*.out
	@rm $(PROJ_2)/*.out
	@rm $(PROJ_1)/*.out
