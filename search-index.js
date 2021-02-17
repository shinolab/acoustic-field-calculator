var searchIndex = JSON.parse('{\
"acoustic_field_calculator":{"doc":"","i":[[0,"calculator","acoustic_field_calculator","Calculator",null,null],[3,"CpuCalculator","acoustic_field_calculator::calculator","Normal Calculator",null,null],[8,"FieldCalculator","","Calculate field at observe area",null,null],[10,"calculate","","",0,[[]]],[0,"accurate","acoustic_field_calculator","accurate mode",null,null],[3,"AccurateCalculator","acoustic_field_calculator::accurate","Accurate Calculator",null,null],[0,"gpu","acoustic_field_calculator","gpu modules",null,null],[8,"GpuFieldBuffer","acoustic_field_calculator::gpu","Calculate field by gpu calculator",null,null],[10,"calculate_field","","",1,[[["arc",3],["queue",3],["device",3],["arc",3]]]],[3,"GpuCalculator","","GPU Calculator",null,null],[8,"SizedArea","","",null,null],[10,"size","","",2,[[]]],[8,"GpuPropagationMedium","","",null,null],[10,"wavenums","","",3,[[]]],[10,"attenuations","","",3,[[]]],[10,"directivities","","",3,[[],["vec",3]]],[0,"field_buffer","acoustic_field_calculator","Field buffer defines what kind of field to calculate and …",null,null],[3,"ComplexPressureField","acoustic_field_calculator::field_buffer","Complex pressure field",null,null],[3,"PowerField","","Power field",null,null],[3,"PressureField","","Pressure field",null,null],[8,"FieldBuffer","","",null,null],[10,"buffer","","",4,[[]]],[10,"buffer_mut","","",4,[[],["vec",3]]],[10,"calc_from_complex_pressure","","",4,[[["complex",6]]]],[8,"ScalarFieldBuffer","","",null,null],[10,"max_result","","",5,[[]]],[0,"fmath","acoustic_field_calculator","provide functions of fast math calculation",null,null],[5,"exp","acoustic_field_calculator::fmath","",null,[[["float",6]],["float",6]]],[5,"sub","","",null,[[["vector3",6]],["vector3",6]]],[5,"acos","","",null,[[["float",6]],["float",6]]],[0,"observe_area","acoustic_field_calculator","Observe area is a set of observing points",null,null],[0,"grid","acoustic_field_calculator::observe_area","Observation points on the grid",null,null],[3,"N1","acoustic_field_calculator::observe_area::grid","",null,null],[3,"N2","","",null,null],[3,"N3","","",null,null],[3,"GridAreaBuilder","","Builder for GridArea",null,null],[4,"Dimension","","",null,null],[13,"None","","",6,null],[13,"One","","",6,null],[13,"Two","","",6,null],[13,"Three","","",6,null],[4,"Axis","","",null,null],[13,"None","","",7,null],[13,"X","","",7,null],[13,"Y","","",7,null],[13,"Z","","",7,null],[3,"GridArea","","Observation points on grid",null,null],[0,"scatter","acoustic_field_calculator::observe_area","Scatter observation points",null,null],[3,"ScatterArea","acoustic_field_calculator::observe_area::scatter","Scatter observation points",null,null],[3,"GridArea","acoustic_field_calculator::observe_area","Observation points on grid",null,null],[3,"GridAreaBuilder","","Builder for GridArea",null,null],[3,"ScatterArea","","Scatter observation points",null,null],[8,"ObserveArea","","",null,null],[10,"points","","Returns all observation points",8,[[]]],[0,"prelude","acoustic_field_calculator","",null,null],[3,"SphereWaveSource","acoustic_field_calculator::prelude","Wave source which emits a simple spherical wave",null,null],[3,"T4010A1","","Ultrasound transducer T4010A1 model",null,null],[8,"WaveSource","","",null,null],[10,"directivity","","Get directivity",9,[[["float",6]],["float",6]]],[10,"frequency","","",9,[[],["float",6]]],[10,"position","","",9,[[],["vector3",6]]],[10,"set_position","","",9,[[["vector3",6]]]],[10,"direction","","",9,[[],["vector3",6]]],[10,"set_direction","","",9,[[["vector3",6]]]],[10,"phase","","",9,[[],["float",6]]],[10,"set_phase","","",9,[[["float",6]]]],[10,"amp","","",9,[[],["float",6]]],[10,"set_amp","","",9,[[["float",6]]]],[6,"Complex","","Complex number",null,null],[6,"Float","","Floating-point number",null,null],[6,"Vector3","","Three-dimensional vector",null,null],[17,"PI","","",null,null],[0,"system","acoustic_field_calculator","System is an environment where wave sources are located. …",null,null],[8,"WaveSourceContainer","acoustic_field_calculator::system","",null,null],[10,"wave_sources","","Returns all of the wave sources",10,[[]]],[10,"wave_sources_mut","","Returns all of the wave sources as mutable",10,[[],["vec",3]]],[10,"add_wave_source","","Add new wave source",10,[[]]],[11,"add_wave_sources","","Add new wave sources",10,[[["vec",3]]]],[8,"PropagationMedium","","",null,null],[10,"propagate","","",11,[[["vector3",6]],["complex",6]]],[3,"UniformSystem","","",null,null],[0,"wave_sources","acoustic_field_calculator","Wave sources",null,null],[3,"SphereWaveSource","acoustic_field_calculator::wave_sources","Wave source which emits a simple spherical wave",null,null],[3,"T4010A1","","Ultrasound transducer T4010A1 model",null,null],[8,"WaveSource","","",null,null],[10,"directivity","","Get directivity",9,[[["float",6]],["float",6]]],[10,"frequency","","",9,[[],["float",6]]],[10,"position","","",9,[[],["vector3",6]]],[10,"set_position","","",9,[[["vector3",6]]]],[10,"direction","","",9,[[],["vector3",6]]],[10,"set_direction","","",9,[[["vector3",6]]]],[10,"phase","","",9,[[],["float",6]]],[10,"set_phase","","",9,[[["float",6]]]],[10,"amp","","",9,[[],["float",6]]],[10,"set_amp","","",9,[[["float",6]]]],[0,"attenuation","acoustic_field_calculator","utilities for atmospheric attenuation",null,null],[5,"attenuation_coef","acoustic_field_calculator::attenuation","Returns an attenuation coefficients due to atmospheric …",null,[[["float",6]],["float",6]]],[0,"sound_speed","acoustic_field_calculator","utilities for sound speed for air",null,null],[5,"calc_sound_speed","acoustic_field_calculator::sound_speed","Calculates sound speed from temperature",null,[[["float",6]],["float",6]]],[0,"wave_sources","acoustic_field_calculator","Wave sources",null,null],[3,"SphereWaveSource","acoustic_field_calculator::wave_sources","Wave source which emits a simple spherical wave",null,null],[3,"T4010A1","","Ultrasound transducer T4010A1 model",null,null],[8,"WaveSource","","",null,null],[10,"directivity","","Get directivity",9,[[["float",6]],["float",6]]],[10,"frequency","","",9,[[],["float",6]]],[10,"position","","",9,[[],["vector3",6]]],[10,"set_position","","",9,[[["vector3",6]]]],[10,"direction","","",9,[[],["vector3",6]]],[10,"set_direction","","",9,[[["vector3",6]]]],[10,"phase","","",9,[[],["float",6]]],[10,"set_phase","","",9,[[["float",6]]]],[10,"amp","","",9,[[],["float",6]]],[10,"set_amp","","",9,[[["float",6]]]],[6,"Float","acoustic_field_calculator","Floating-point number",null,null],[17,"PI","","",null,null],[6,"Vector3","","Three-dimensional vector",null,null],[6,"Complex","","Complex number",null,null],[11,"from","acoustic_field_calculator::calculator","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"to_subset","","",12,[[],["option",4]]],[11,"is_in_subset","","",12,[[]]],[11,"to_subset_unchecked","","",12,[[]]],[11,"from_subset","","",12,[[]]],[11,"vzip","","",12,[[]]],[11,"init","","",12,[[]]],[11,"deref","","",12,[[]]],[11,"deref_mut","","",12,[[]]],[11,"drop","","",12,[[]]],[11,"ref_from_ptr","","",12,[[],["option",4]]],[11,"is_size_suitable","","",12,[[]]],[11,"indiv_size","","",12,[[]]],[11,"from","acoustic_field_calculator::prelude","",13,[[]]],[11,"into","","",13,[[]]],[11,"to_owned","","",13,[[]]],[11,"clone_into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"to_subset","","",13,[[],["option",4]]],[11,"is_in_subset","","",13,[[]]],[11,"to_subset_unchecked","","",13,[[]]],[11,"from_subset","","",13,[[]]],[11,"vzip","","",13,[[]]],[11,"init","","",13,[[]]],[11,"deref","","",13,[[]]],[11,"deref_mut","","",13,[[]]],[11,"drop","","",13,[[]]],[11,"ref_from_ptr","","",13,[[],["option",4]]],[11,"is_size_suitable","","",13,[[]]],[11,"indiv_size","","",13,[[]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"to_subset","","",14,[[],["option",4]]],[11,"is_in_subset","","",14,[[]]],[11,"to_subset_unchecked","","",14,[[]]],[11,"from_subset","","",14,[[]]],[11,"vzip","","",14,[[]]],[11,"init","","",14,[[]]],[11,"deref","","",14,[[]]],[11,"deref_mut","","",14,[[]]],[11,"drop","","",14,[[]]],[11,"ref_from_ptr","","",14,[[],["option",4]]],[11,"is_size_suitable","","",14,[[]]],[11,"indiv_size","","",14,[[]]],[11,"from","acoustic_field_calculator::accurate","",15,[[]]],[11,"into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"to_subset","","",15,[[],["option",4]]],[11,"is_in_subset","","",15,[[]]],[11,"to_subset_unchecked","","",15,[[]]],[11,"from_subset","","",15,[[]]],[11,"vzip","","",15,[[]]],[11,"init","","",15,[[]]],[11,"deref","","",15,[[]]],[11,"deref_mut","","",15,[[]]],[11,"drop","","",15,[[]]],[11,"ref_from_ptr","","",15,[[],["option",4]]],[11,"is_size_suitable","","",15,[[]]],[11,"indiv_size","","",15,[[]]],[11,"from","acoustic_field_calculator::gpu","",16,[[]]],[11,"into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"to_subset","","",16,[[],["option",4]]],[11,"is_in_subset","","",16,[[]]],[11,"to_subset_unchecked","","",16,[[]]],[11,"from_subset","","",16,[[]]],[11,"vzip","","",16,[[]]],[11,"init","","",16,[[]]],[11,"deref","","",16,[[]]],[11,"deref_mut","","",16,[[]]],[11,"drop","","",16,[[]]],[11,"ref_from_ptr","","",16,[[],["option",4]]],[11,"is_size_suitable","","",16,[[]]],[11,"indiv_size","","",16,[[]]],[11,"from","acoustic_field_calculator::field_buffer","",17,[[]]],[11,"into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"to_subset","","",17,[[],["option",4]]],[11,"is_in_subset","","",17,[[]]],[11,"to_subset_unchecked","","",17,[[]]],[11,"from_subset","","",17,[[]]],[11,"vzip","","",17,[[]]],[11,"init","","",17,[[]]],[11,"deref","","",17,[[]]],[11,"deref_mut","","",17,[[]]],[11,"drop","","",17,[[]]],[11,"ref_from_ptr","","",17,[[],["option",4]]],[11,"is_size_suitable","","",17,[[]]],[11,"indiv_size","","",17,[[]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"to_subset","","",18,[[],["option",4]]],[11,"is_in_subset","","",18,[[]]],[11,"to_subset_unchecked","","",18,[[]]],[11,"from_subset","","",18,[[]]],[11,"vzip","","",18,[[]]],[11,"init","","",18,[[]]],[11,"deref","","",18,[[]]],[11,"deref_mut","","",18,[[]]],[11,"drop","","",18,[[]]],[11,"ref_from_ptr","","",18,[[],["option",4]]],[11,"is_size_suitable","","",18,[[]]],[11,"indiv_size","","",18,[[]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"to_subset","","",19,[[],["option",4]]],[11,"is_in_subset","","",19,[[]]],[11,"to_subset_unchecked","","",19,[[]]],[11,"from_subset","","",19,[[]]],[11,"vzip","","",19,[[]]],[11,"init","","",19,[[]]],[11,"deref","","",19,[[]]],[11,"deref_mut","","",19,[[]]],[11,"drop","","",19,[[]]],[11,"ref_from_ptr","","",19,[[],["option",4]]],[11,"is_size_suitable","","",19,[[]]],[11,"indiv_size","","",19,[[]]],[11,"from","acoustic_field_calculator::observe_area::grid","",20,[[]]],[11,"into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"to_subset","","",20,[[],["option",4]]],[11,"is_in_subset","","",20,[[]]],[11,"to_subset_unchecked","","",20,[[]]],[11,"from_subset","","",20,[[]]],[11,"vzip","","",20,[[]]],[11,"init","","",20,[[]]],[11,"deref","","",20,[[]]],[11,"deref_mut","","",20,[[]]],[11,"drop","","",20,[[]]],[11,"ref_from_ptr","","",20,[[],["option",4]]],[11,"is_size_suitable","","",20,[[]]],[11,"indiv_size","","",20,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_owned","","",6,[[]]],[11,"clone_into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"inlined_clone","","",6,[[]]],[11,"to_subset","","",6,[[],["option",4]]],[11,"is_in_subset","","",6,[[]]],[11,"to_subset_unchecked","","",6,[[]]],[11,"from_subset","","",6,[[]]],[11,"vzip","","",6,[[]]],[11,"init","","",6,[[]]],[11,"deref","","",6,[[]]],[11,"deref_mut","","",6,[[]]],[11,"drop","","",6,[[]]],[11,"ref_from_ptr","","",6,[[],["option",4]]],[11,"is_size_suitable","","",6,[[]]],[11,"indiv_size","","",6,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"to_owned","","",7,[[]]],[11,"clone_into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"inlined_clone","","",7,[[]]],[11,"to_subset","","",7,[[],["option",4]]],[11,"is_in_subset","","",7,[[]]],[11,"to_subset_unchecked","","",7,[[]]],[11,"from_subset","","",7,[[]]],[11,"vzip","","",7,[[]]],[11,"init","","",7,[[]]],[11,"deref","","",7,[[]]],[11,"deref_mut","","",7,[[]]],[11,"drop","","",7,[[]]],[11,"ref_from_ptr","","",7,[[],["option",4]]],[11,"is_size_suitable","","",7,[[]]],[11,"indiv_size","","",7,[[]]],[11,"from","","",21,[[]]],[11,"into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"to_subset","","",21,[[],["option",4]]],[11,"is_in_subset","","",21,[[]]],[11,"to_subset_unchecked","","",21,[[]]],[11,"from_subset","","",21,[[]]],[11,"vzip","","",21,[[]]],[11,"init","","",21,[[]]],[11,"deref","","",21,[[]]],[11,"deref_mut","","",21,[[]]],[11,"drop","","",21,[[]]],[11,"ref_from_ptr","","",21,[[],["option",4]]],[11,"is_size_suitable","","",21,[[]]],[11,"indiv_size","","",21,[[]]],[11,"from","","",22,[[]]],[11,"into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"to_subset","","",22,[[],["option",4]]],[11,"is_in_subset","","",22,[[]]],[11,"to_subset_unchecked","","",22,[[]]],[11,"from_subset","","",22,[[]]],[11,"vzip","","",22,[[]]],[11,"init","","",22,[[]]],[11,"deref","","",22,[[]]],[11,"deref_mut","","",22,[[]]],[11,"drop","","",22,[[]]],[11,"ref_from_ptr","","",22,[[],["option",4]]],[11,"is_size_suitable","","",22,[[]]],[11,"indiv_size","","",22,[[]]],[11,"from","","",23,[[]]],[11,"into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"to_subset","","",23,[[],["option",4]]],[11,"is_in_subset","","",23,[[]]],[11,"to_subset_unchecked","","",23,[[]]],[11,"from_subset","","",23,[[]]],[11,"vzip","","",23,[[]]],[11,"init","","",23,[[]]],[11,"deref","","",23,[[]]],[11,"deref_mut","","",23,[[]]],[11,"drop","","",23,[[]]],[11,"ref_from_ptr","","",23,[[],["option",4]]],[11,"is_size_suitable","","",23,[[]]],[11,"indiv_size","","",23,[[]]],[11,"from","","",24,[[]]],[11,"into","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"to_subset","","",24,[[],["option",4]]],[11,"is_in_subset","","",24,[[]]],[11,"to_subset_unchecked","","",24,[[]]],[11,"from_subset","","",24,[[]]],[11,"vzip","","",24,[[]]],[11,"init","","",24,[[]]],[11,"deref","","",24,[[]]],[11,"deref_mut","","",24,[[]]],[11,"drop","","",24,[[]]],[11,"ref_from_ptr","","",24,[[],["option",4]]],[11,"is_size_suitable","","",24,[[]]],[11,"indiv_size","","",24,[[]]],[11,"from","acoustic_field_calculator::observe_area::scatter","",25,[[]]],[11,"into","","",25,[[]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"to_subset","","",25,[[],["option",4]]],[11,"is_in_subset","","",25,[[]]],[11,"to_subset_unchecked","","",25,[[]]],[11,"from_subset","","",25,[[]]],[11,"vzip","","",25,[[]]],[11,"init","","",25,[[]]],[11,"deref","","",25,[[]]],[11,"deref_mut","","",25,[[]]],[11,"drop","","",25,[[]]],[11,"ref_from_ptr","","",25,[[],["option",4]]],[11,"is_size_suitable","","",25,[[]]],[11,"indiv_size","","",25,[[]]],[11,"from","acoustic_field_calculator::system","",26,[[]]],[11,"into","","",26,[[]]],[11,"borrow","","",26,[[]]],[11,"borrow_mut","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"try_into","","",26,[[],["result",4]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"to_subset","","",26,[[],["option",4]]],[11,"is_in_subset","","",26,[[]]],[11,"to_subset_unchecked","","",26,[[]]],[11,"from_subset","","",26,[[]]],[11,"vzip","","",26,[[]]],[11,"init","","",26,[[]]],[11,"deref","","",26,[[]]],[11,"deref_mut","","",26,[[]]],[11,"drop","","",26,[[]]],[11,"ref_from_ptr","","",26,[[],["option",4]]],[11,"is_size_suitable","","",26,[[]]],[11,"indiv_size","","",26,[[]]],[11,"calculate","acoustic_field_calculator::calculator","",12,[[]]],[11,"calculate","acoustic_field_calculator::accurate","",15,[[]]],[11,"calculate","acoustic_field_calculator::gpu","",16,[[]]],[11,"calculate_field","acoustic_field_calculator::field_buffer","",17,[[["arc",3],["queue",3],["device",3],["arc",3]]]],[11,"calculate_field","","",18,[[["arc",3],["queue",3],["device",3],["arc",3]]]],[11,"calculate_field","","",19,[[["arc",3],["queue",3],["device",3],["arc",3]]]],[11,"size","acoustic_field_calculator::observe_area::grid","",21,[[]]],[11,"size","acoustic_field_calculator::observe_area::scatter","",25,[[]]],[11,"wavenums","acoustic_field_calculator::system","",26,[[]]],[11,"attenuations","","",26,[[]]],[11,"directivities","","",26,[[],["vec",3]]],[11,"calc_from_complex_pressure","acoustic_field_calculator::field_buffer","",17,[[["complex",6]],["complex",6]]],[11,"buffer","","",17,[[]]],[11,"buffer_mut","","",17,[[],["vec",3]]],[11,"calc_from_complex_pressure","","",18,[[["complex",6]],["float",6]]],[11,"buffer","","",18,[[]]],[11,"buffer_mut","","",18,[[],["vec",3]]],[11,"calc_from_complex_pressure","","",19,[[["complex",6]],["float",6]]],[11,"buffer","","",19,[[]]],[11,"buffer_mut","","",19,[[],["vec",3]]],[11,"points","acoustic_field_calculator::observe_area::grid","",21,[[]]],[11,"points","acoustic_field_calculator::observe_area::scatter","",25,[[]]],[11,"wave_sources","acoustic_field_calculator::system","",26,[[]]],[11,"wave_sources_mut","","",26,[[],["vec",3]]],[11,"add_wave_source","","",26,[[]]],[11,"propagate","","",26,[[["vector3",6]],["complex",6]]],[11,"directivity","acoustic_field_calculator::prelude","",13,[[["float",6]],["float",6]]],[11,"frequency","","",13,[[],["float",6]]],[11,"position","","",13,[[],["vector3",6]]],[11,"set_position","","",13,[[["vector3",6]]]],[11,"phase","","",13,[[],["float",6]]],[11,"set_phase","","",13,[[["float",6]]]],[11,"amp","","",13,[[],["float",6]]],[11,"set_amp","","",13,[[["float",6]]]],[11,"direction","","",13,[[],["vector3",6]]],[11,"set_direction","","",13,[[["vector3",6]]]],[11,"directivity","","",14,[[["float",6]],["float",6]]],[11,"frequency","","",14,[[],["float",6]]],[11,"position","","",14,[[],["vector3",6]]],[11,"set_position","","",14,[[["vector3",6]]]],[11,"phase","","",14,[[],["float",6]]],[11,"set_phase","","",14,[[["float",6]]]],[11,"amp","","",14,[[],["float",6]]],[11,"set_amp","","",14,[[["float",6]]]],[11,"direction","","",14,[[],["vector3",6]]],[11,"set_direction","","",14,[[["vector3",6]]]],[11,"clone","","",13,[[],["spherewavesource",3]]],[11,"clone","","",14,[[],["t4010a1",3]]],[11,"clone","acoustic_field_calculator::observe_area::grid","",6,[[],["dimension",4]]],[11,"clone","","",7,[[],["axis",4]]],[11,"default","acoustic_field_calculator::calculator","",12,[[]]],[11,"default","acoustic_field_calculator::accurate","",15,[[]]],[11,"default","acoustic_field_calculator::gpu","",16,[[]]],[11,"default","acoustic_field_calculator::field_buffer","",17,[[]]],[11,"default","","",18,[[]]],[11,"default","","",19,[[]]],[11,"default","acoustic_field_calculator::observe_area::grid","",20,[[]]],[11,"default","acoustic_field_calculator::observe_area::scatter","",25,[[]]],[11,"eq","acoustic_field_calculator::observe_area::grid","",6,[[["dimension",4]]]],[11,"ne","","",6,[[["dimension",4]]]],[11,"eq","","",7,[[["axis",4]]]],[11,"fmt","acoustic_field_calculator::prelude","",13,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","acoustic_field_calculator::observe_area::grid","",6,[[["formatter",3]],["result",6]]],[11,"fmt","","",7,[[["formatter",3]],["result",6]]],[11,"new","acoustic_field_calculator::calculator","",12,[[]]],[11,"new","acoustic_field_calculator::prelude","Returns a SphereWaveSource",13,[[["vector3",6],["float",6]]]],[11,"new","","Returns a T4010A1 transducer",14,[[["vector3",6],["float",6]]]],[11,"new","acoustic_field_calculator::accurate","",15,[[]]],[11,"new","acoustic_field_calculator::gpu","",16,[[]]],[11,"new","acoustic_field_calculator::field_buffer","",17,[[]]],[11,"new","","",18,[[]]],[11,"new","","",19,[[]]],[11,"new","acoustic_field_calculator::observe_area::grid","",20,[[]]],[11,"x_range","","Set observation range along x-axis",20,[[["float",6]],[["used",3],["gridareabuilder",3]]]],[11,"x_at","","Set observation point on x-axis",20,[[["float",6]],[["gridareabuilder",3],["unused",3]]]],[11,"y_range","","Set observation range along y-axis",20,[[["float",6]],[["gridareabuilder",3],["used",3]]]],[11,"y_at","","Set observation point on y-axis",20,[[["float",6]],[["unused",3],["gridareabuilder",3]]]],[11,"z_range","","Set observation range along z-axis",20,[[["float",6]],[["gridareabuilder",3],["used",3]]]],[11,"z_at","","Set observation point on z-axis",20,[[["float",6]],[["gridareabuilder",3],["unused",3]]]],[11,"resolution","","Set resolution, that is spacing of the Grid",20,[[["float",6]],[["gridareabuilder",3],["used",3]]]],[11,"generate","","",20,[[],[["n1",3],["gridarea",3]]]],[11,"generate","","",20,[[],[["n1",3],["gridarea",3]]]],[11,"generate","","",20,[[],[["n1",3],["gridarea",3]]]],[11,"generate","","",20,[[],[["n2",3],["gridarea",3]]]],[11,"generate","","",20,[[],[["n2",3],["gridarea",3]]]],[11,"generate","","",20,[[],[["n2",3],["gridarea",3]]]],[11,"generate","","Generate three-dimensional GridArea. You must specify the …",20,[[],[["n3",3],["gridarea",3]]]],[11,"from_i32","","",7,[[]]],[11,"dimension","","Returns a dimension",21,[[],["dimension",4]]],[11,"bounds","","Returns a bounds",21,[[],["bounds",3]]],[11,"new","acoustic_field_calculator::observe_area::scatter","",25,[[]]],[11,"add_observe_point","","Add a new observation point",25,[[["vector3",6]]]],[11,"add_wave_sources","acoustic_field_calculator::system","Add new wave sources",10,[[["vec",3]]]],[11,"new","","",26,[[["float",6]]]],[11,"add_wave_source_with_wavenum","","",26,[[["float",6]]]],[11,"add_wave_source_with_atten","","",26,[[["float",6]]]],[11,"add_wave_source_with_wavenum_and_atten","","",26,[[["float",6]]]],[11,"wavenums","","",26,[[]]],[11,"attens","","",26,[[]]],[11,"sound_speed","","",26,[[],["float",6]]],[11,"info","","",26,[[],["string",3]]],[11,"info_of_source","","",26,[[],["string",3]]]],"p":[[8,"FieldCalculator"],[8,"GpuFieldBuffer"],[8,"SizedArea"],[8,"GpuPropagationMedium"],[8,"FieldBuffer"],[8,"ScalarFieldBuffer"],[4,"Dimension"],[4,"Axis"],[8,"ObserveArea"],[8,"WaveSource"],[8,"WaveSourceContainer"],[8,"PropagationMedium"],[3,"CpuCalculator"],[3,"SphereWaveSource"],[3,"T4010A1"],[3,"AccurateCalculator"],[3,"GpuCalculator"],[3,"ComplexPressureField"],[3,"PowerField"],[3,"PressureField"],[3,"GridAreaBuilder"],[3,"GridArea"],[3,"N1"],[3,"N2"],[3,"N3"],[3,"ScatterArea"],[3,"UniformSystem"]]},\
"acoustic_field_optimizer":{"doc":"","i":[[0,"multiple_foci","acoustic_field_optimizer","Producing multiple foci",null,null],[0,"macros","acoustic_field_optimizer::multiple_foci","",null,null],[5,"propagate","acoustic_field_optimizer::multiple_foci::macros","",null,[[["vector3",6],["float",6]],["complex",6]]],[5,"generate_propagation_matrix","","",null,[[["uniformsystem",3]],[["dynamic",3],["complex",6],["vecstorage",3],["matrix",3]]]],[5,"append_matrix_col","","",null,[[["dynamic",3],["complex",6],["vecstorage",3],["matrix",3],["matrix",3]],[["dynamic",3],["complex",6],["vecstorage",3],["matrix",3]]]],[3,"IFFT","acoustic_field_optimizer::multiple_foci","Inverse FFT",null,null],[3,"GS","","Gerchberg-Saxton",null,null],[3,"GSPAT","","GS-PAT",null,null],[3,"Naive","","Naive linear synthesis",null,null],[3,"Horn","","Inoue et al., 2015",null,null],[3,"Long","","Long et al., 2014",null,null],[3,"APO","","Acoustic Power Optimization",null,null],[3,"GaussNewton","","Gauss-Newton",null,null],[3,"GradientDescent","","Gradient descent",null,null],[3,"LM","","Levenberg-Marquardt",null,null],[3,"BesselBeam","acoustic_field_optimizer","Producing a bessel beam",null,null],[3,"FocalPoint","","Producing a single focal point",null,null],[8,"Optimizer","","",null,null],[10,"optimize","","",0,[[["uniformsystem",3]]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"to_subset","","",1,[[],["option",4]]],[11,"is_in_subset","","",1,[[]]],[11,"to_subset_unchecked","","",1,[[]]],[11,"from_subset","","",1,[[]]],[11,"vzip","","",1,[[]]],[11,"init","","",1,[[]]],[11,"deref","","",1,[[]]],[11,"deref_mut","","",1,[[]]],[11,"drop","","",1,[[]]],[11,"ref_from_ptr","","",1,[[],["option",4]]],[11,"is_size_suitable","","",1,[[]]],[11,"indiv_size","","",1,[[]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"to_subset","","",2,[[],["option",4]]],[11,"is_in_subset","","",2,[[]]],[11,"to_subset_unchecked","","",2,[[]]],[11,"from_subset","","",2,[[]]],[11,"vzip","","",2,[[]]],[11,"init","","",2,[[]]],[11,"deref","","",2,[[]]],[11,"deref_mut","","",2,[[]]],[11,"drop","","",2,[[]]],[11,"ref_from_ptr","","",2,[[],["option",4]]],[11,"is_size_suitable","","",2,[[]]],[11,"indiv_size","","",2,[[]]],[11,"from","acoustic_field_optimizer::multiple_foci","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"to_subset","","",3,[[],["option",4]]],[11,"is_in_subset","","",3,[[]]],[11,"to_subset_unchecked","","",3,[[]]],[11,"from_subset","","",3,[[]]],[11,"vzip","","",3,[[]]],[11,"init","","",3,[[]]],[11,"deref","","",3,[[]]],[11,"deref_mut","","",3,[[]]],[11,"drop","","",3,[[]]],[11,"ref_from_ptr","","",3,[[],["option",4]]],[11,"is_size_suitable","","",3,[[]]],[11,"indiv_size","","",3,[[]]],[11,"from","","",4,[[]]],[11,"into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"to_subset","","",4,[[],["option",4]]],[11,"is_in_subset","","",4,[[]]],[11,"to_subset_unchecked","","",4,[[]]],[11,"from_subset","","",4,[[]]],[11,"vzip","","",4,[[]]],[11,"init","","",4,[[]]],[11,"deref","","",4,[[]]],[11,"deref_mut","","",4,[[]]],[11,"drop","","",4,[[]]],[11,"ref_from_ptr","","",4,[[],["option",4]]],[11,"is_size_suitable","","",4,[[]]],[11,"indiv_size","","",4,[[]]],[11,"from","","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"to_subset","","",5,[[],["option",4]]],[11,"is_in_subset","","",5,[[]]],[11,"to_subset_unchecked","","",5,[[]]],[11,"from_subset","","",5,[[]]],[11,"vzip","","",5,[[]]],[11,"init","","",5,[[]]],[11,"deref","","",5,[[]]],[11,"deref_mut","","",5,[[]]],[11,"drop","","",5,[[]]],[11,"ref_from_ptr","","",5,[[],["option",4]]],[11,"is_size_suitable","","",5,[[]]],[11,"indiv_size","","",5,[[]]],[11,"from","","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"to_subset","","",6,[[],["option",4]]],[11,"is_in_subset","","",6,[[]]],[11,"to_subset_unchecked","","",6,[[]]],[11,"from_subset","","",6,[[]]],[11,"vzip","","",6,[[]]],[11,"init","","",6,[[]]],[11,"deref","","",6,[[]]],[11,"deref_mut","","",6,[[]]],[11,"drop","","",6,[[]]],[11,"ref_from_ptr","","",6,[[],["option",4]]],[11,"is_size_suitable","","",6,[[]]],[11,"indiv_size","","",6,[[]]],[11,"from","","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"to_subset","","",7,[[],["option",4]]],[11,"is_in_subset","","",7,[[]]],[11,"to_subset_unchecked","","",7,[[]]],[11,"from_subset","","",7,[[]]],[11,"vzip","","",7,[[]]],[11,"init","","",7,[[]]],[11,"deref","","",7,[[]]],[11,"deref_mut","","",7,[[]]],[11,"drop","","",7,[[]]],[11,"ref_from_ptr","","",7,[[],["option",4]]],[11,"is_size_suitable","","",7,[[]]],[11,"indiv_size","","",7,[[]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"to_subset","","",8,[[],["option",4]]],[11,"is_in_subset","","",8,[[]]],[11,"to_subset_unchecked","","",8,[[]]],[11,"from_subset","","",8,[[]]],[11,"vzip","","",8,[[]]],[11,"init","","",8,[[]]],[11,"deref","","",8,[[]]],[11,"deref_mut","","",8,[[]]],[11,"drop","","",8,[[]]],[11,"ref_from_ptr","","",8,[[],["option",4]]],[11,"is_size_suitable","","",8,[[]]],[11,"indiv_size","","",8,[[]]],[11,"from","","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"to_subset","","",9,[[],["option",4]]],[11,"is_in_subset","","",9,[[]]],[11,"to_subset_unchecked","","",9,[[]]],[11,"from_subset","","",9,[[]]],[11,"vzip","","",9,[[]]],[11,"init","","",9,[[]]],[11,"deref","","",9,[[]]],[11,"deref_mut","","",9,[[]]],[11,"drop","","",9,[[]]],[11,"ref_from_ptr","","",9,[[],["option",4]]],[11,"is_size_suitable","","",9,[[]]],[11,"indiv_size","","",9,[[]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"to_subset","","",10,[[],["option",4]]],[11,"is_in_subset","","",10,[[]]],[11,"to_subset_unchecked","","",10,[[]]],[11,"from_subset","","",10,[[]]],[11,"vzip","","",10,[[]]],[11,"init","","",10,[[]]],[11,"deref","","",10,[[]]],[11,"deref_mut","","",10,[[]]],[11,"drop","","",10,[[]]],[11,"ref_from_ptr","","",10,[[],["option",4]]],[11,"is_size_suitable","","",10,[[]]],[11,"indiv_size","","",10,[[]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"to_subset","","",11,[[],["option",4]]],[11,"is_in_subset","","",11,[[]]],[11,"to_subset_unchecked","","",11,[[]]],[11,"from_subset","","",11,[[]]],[11,"vzip","","",11,[[]]],[11,"init","","",11,[[]]],[11,"deref","","",11,[[]]],[11,"deref_mut","","",11,[[]]],[11,"drop","","",11,[[]]],[11,"ref_from_ptr","","",11,[[],["option",4]]],[11,"is_size_suitable","","",11,[[]]],[11,"indiv_size","","",11,[[]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"to_subset","","",12,[[],["option",4]]],[11,"is_in_subset","","",12,[[]]],[11,"to_subset_unchecked","","",12,[[]]],[11,"from_subset","","",12,[[]]],[11,"vzip","","",12,[[]]],[11,"init","","",12,[[]]],[11,"deref","","",12,[[]]],[11,"deref_mut","","",12,[[]]],[11,"drop","","",12,[[]]],[11,"ref_from_ptr","","",12,[[],["option",4]]],[11,"is_size_suitable","","",12,[[]]],[11,"indiv_size","","",12,[[]]],[11,"optimize","acoustic_field_optimizer","",1,[[["uniformsystem",3]]]],[11,"optimize","","",2,[[["uniformsystem",3]]]],[11,"optimize","acoustic_field_optimizer::multiple_foci","",3,[[["uniformsystem",3]]]],[11,"optimize","","",4,[[["uniformsystem",3]]]],[11,"optimize","","",5,[[["uniformsystem",3]]]],[11,"optimize","","",6,[[["uniformsystem",3]]]],[11,"optimize","","",7,[[["uniformsystem",3]]]],[11,"optimize","","",8,[[["uniformsystem",3]]]],[11,"optimize","","",9,[[["uniformsystem",3]]]],[11,"optimize","","",10,[[["uniformsystem",3]]]],[11,"optimize","","",11,[[["uniformsystem",3]]]],[11,"optimize","","",12,[[["uniformsystem",3]]]],[11,"new","acoustic_field_optimizer","",1,[[["vector3",6],["float",6]]]],[11,"new","","",2,[[["vector3",6]]]],[11,"new","acoustic_field_optimizer::multiple_foci","",3,[[["vector3",6],["float",6]]]],[11,"new","","",4,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"new","","",5,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"new","","",6,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"new","","",7,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"set_repeat","","",7,[[]]],[11,"set_lambda","","",7,[[["float",6]]]],[11,"set_tikhonov_param","","",7,[[["float",6]]]],[11,"new","","",8,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"new","","",9,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"new","","",10,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"new","","",11,[[["vector3",6],["vec",3],["vec",3],["float",6]]]],[11,"new","","",12,[[["vector3",6],["vec",3],["vec",3],["float",6]]]]],"p":[[8,"Optimizer"],[3,"BesselBeam"],[3,"FocalPoint"],[3,"IFFT"],[3,"GS"],[3,"GSPAT"],[3,"Naive"],[3,"Horn"],[3,"Long"],[3,"APO"],[3,"GaussNewton"],[3,"GradientDescent"],[3,"LM"]]},\
"afccapi":{"doc":"","i":[[0,"calculator","afccapi","",null,null],[5,"AFC_CreateCalculator","afccapi::calculator","",null,[[]]],[5,"AFC_FreeCalculator","","",null,[[]]],[5,"AFC_Calculate","","",null,[[]]],[0,"field_buffer","afccapi","",null,null],[5,"AFC_CreatePressureField","afccapi::field_buffer","",null,[[]]],[5,"AFC_FreePressureField","","",null,[[]]],[5,"AFC_CreatePowerField","","",null,[[]]],[5,"AFC_FreePowerField","","",null,[[]]],[5,"AFC_CreateComplexPressureField","","",null,[[]]],[5,"AFC_FreeComplexPressureField","","",null,[[]]],[0,"observe_area","afccapi","",null,null],[5,"AFC_CreateScatterArea","afccapi::observe_area","",null,[[]]],[5,"AFC_ScatterAddObservePoint","","",null,[[["vector3",6]]]],[5,"AFC_FreeScatterArea","","",null,[[]]],[5,"AFC_CreateGridArea1D","","",null,[[]]],[5,"AFC_CreateGridArea2D","","",null,[[]]],[5,"AFC_CreateGridArea3D","","",null,[[]]],[5,"AFC_FreeGridArea","","",null,[[]]],[5,"AFC_GetGridSize","","",null,[[]]],[0,"optimizer","afccapi","",null,null],[5,"AFO_FocalPoint","afccapi::optimizer","",null,[[["vector3",6]]]],[5,"AFO_BesselBeam","","",null,[[["vector3",6]]]],[5,"AFO_IFFT","","",null,[[["vector3",6]]]],[5,"AFO_GSPAT","","",null,[[]]],[5,"AFO_GS","","",null,[[]]],[5,"AFO_Naive","","",null,[[]]],[5,"AFO_Horn","","",null,[[]]],[5,"AFO_Long","","",null,[[]]],[5,"AFO_APO","","",null,[[]]],[5,"AFO_GaussNewton","","",null,[[]]],[5,"AFO_GradientDescent","","",null,[[]]],[5,"AFO_LM","","",null,[[]]],[0,"system","afccapi","",null,null],[5,"AFC_CreateUniformSystem","afccapi::system","",null,[[]]],[5,"AFC_FreeUniformSystem","","",null,[[]]],[5,"AFC_UniformSystemSoundSpeed","","",null,[[]]],[5,"AFC_UniformSystemInfo","","",null,[[]]],[5,"AFC_UniformSystemSourceInfo","","",null,[[]]],[5,"AFC_AddSphereWaveSource","","",null,[[["spherewavesource",3]]]],[5,"AFC_AddT4010A1","","",null,[[["t4010a1",3]]]],[5,"AFC_GetWaveSources","","",null,[[]]]],"p":[]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);