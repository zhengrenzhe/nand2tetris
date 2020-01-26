HardwareSimulator = ./tools/HardwareSimulator.sh

PROJ_1 = projects/01
PROJ_2 = projects/02
PROJ_3 = projects/03

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