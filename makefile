.PHONY: proj1 proj2 proj3 proj4 proj5 proj7

HardwareSimulator = ./tools/HardwareSimulator.sh
CPUEmulator = ./tools/CPUEmulator.sh
Assembler = ./tools/Assembler.sh

PROJ_1 = projects/01
PROJ_2 = projects/02
PROJ_3 = projects/03
PROJ_4 = projects/04
PROJ_5 = projects/05

proj1: $(PROJ_1)/*.tst
	@for file in $^ ; do \
		echo $${file}; $(HardwareSimulator) $${file} ; \
	done

proj2: $(PROJ_2)/*.tst
	@for file in $^ ; do \
		echo $${file}; $(HardwareSimulator) $${file} ; \
	done

proj3: $(PROJ_3)/**/*.tst
	@for file in $^ ; do \
		echo $${file}; $(HardwareSimulator) $${file} ; \
	done

proj4:
	$(Assembler) $(PROJ_4)/fill/Fill.asm
	@echo
	$(CPUEmulator) $(PROJ_4)/fill/FillAutomatic.tst

	@echo

	$(Assembler) $(PROJ_4)/mult/Mult.asm
	@echo
	$(CPUEmulator) $(PROJ_4)/mult/Mult.tst

proj5:
	$(HardwareSimulator) $(PROJ_5)/CPU-external.tst
	$(HardwareSimulator) $(PROJ_5)/CPU.tst
	$(HardwareSimulator) $(PROJ_5)/ComputerAdd-external.tst
	$(HardwareSimulator) $(PROJ_5)/ComputerAdd.tst
	$(HardwareSimulator) $(PROJ_5)/ComputerMax-external.tst
	$(HardwareSimulator) $(PROJ_5)/ComputerMax.tst
	$(HardwareSimulator) $(PROJ_5)/ComputerRect-external.tst
	$(HardwareSimulator) $(PROJ_5)/ComputerRect.tst

proj7:
	cd compiler \
	&& cargo build --all --release \
	&& cp target/release/vm-compiler ../projects/07/StackArithmetic/SimpleAdd \
	&& cd ../projects/07/StackArithmetic/SimpleAdd \
	&& ./vm-compiler SimpleAdd.vm \
	&& ../../../../tools/CPUEmulator.sh SimpleAdd.tst \
	&& mv ./vm-compiler ../StackTest \
	&& cd ../StackTest \
	&& ./vm-compiler StackTest.vm \
	&& ../../../../tools/CPUEmulator.sh StackTest.tst \
	&& mv ./vm-compiler ../../MemoryAccess/BasicTest/ \
	&& cd ../../MemoryAccess/BasicTest/ \
	&& ./vm-compiler BasicTest.vm \
	&& ../../../../tools/CPUEmulator.sh BasicTest.tst \
	&& mv ./vm-compiler ../PointerTest \
	&& cd ../PointerTest \
	&& ./vm-compiler PointerTest.vm \
	&& ../../../../tools/CPUEmulator.sh PointerTest.tst \
	&& mv ./vm-compiler ../StaticTest \
	&& cd ../StaticTest \
	&& ./vm-compiler StaticTest.vm \
	&& ../../../../tools/CPUEmulator.sh StaticTest.tst \
	&& rm vm-compiler


test: proj1 proj2 proj3 proj4 proj5 proj7
