kotus_sanat.txt: kotus-sanalista/kotus-sanalista_v1.xml.zst
	zstdcat $< | sed -ne 's,.*<s>\(.*\)</s>.*,\1,p' | sort > $@