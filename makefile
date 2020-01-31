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

proj5: $(PROJ_5)/*.tst
	@for file in $^ ; do \
		echo $${file}; $(HardwareSimulator) $${file} ; \
	done

test: proj1 proj2 proj3 proj4
