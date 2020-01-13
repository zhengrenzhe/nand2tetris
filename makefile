HardwareSimulator = ./tools/HardwareSimulator.sh

PROJ_1 = projects/01
PROJ_2 = projects/02

proj1: $(PROJ_1)/*.tst
	@for file in $^ ; do \
		echo $${file}; $(HardwareSimulator) $${file} ; \
	done

proj2: $(PROJ_2)/*.tst
	@for file in $^ ; do \
		echo $${file}; $(HardwareSimulator) $${file} ; \
	done