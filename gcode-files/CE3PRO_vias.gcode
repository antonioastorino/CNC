;FLAVOR:Marlin
;TIME:5
;Filament used: 0.000331543m
;Layer height: 0.5
;MINX:108.416
;MINY:108.416
;MINZ:0.2
;MAXX:111.584
;MAXY:111.584
;MAXZ:0.2
;Generated with Cura_SteamEngine 4.8.0
M140 S50
M105
M190 S50
M104 S200
M105
M109 S200
M82 ;absolute extrusion mode
; Ender 3 Custom Start G-code
G92 E0 ; Reset Extruder
G28 ; Home all axes
G1 Z2.0 F3000 ; Move Z Axis up little to prevent scratching of Heat Bed
G1 X0.1 Y20 Z0.3 F5000.0 ; Move to start position
G1 X0.1 Y200.0 Z0.3 F1500.0 E15 ; Draw the first line
G1 X0.4 Y200.0 Z0.3 F5000.0 ; Move to side a little
G1 X0.4 Y20 Z0.3 F1500.0 E30 ; Draw the second line
G92 E0 ; Reset Extruder
G1 Z2.0 F3000 ; Move Z Axis up little to prevent scratching of Heat Bed
G1 X5 Y20 Z0.3 F5000.0 ; Move over to prevent blob squish
G92 E0
G92 E0
G1 F1500 E-6.5
;LAYER_COUNT:2
;LAYER:0
M107
;MESH:vias.stl
G0 F6000 X110.125 Y111.584 Z0.2
;TYPE:WALL-OUTER
G1 F1500 E0
G1 F600 X109.811 Y111.578 E0.01045
G1 X109.51 Y111.512 E0.02069
G1 X109.227 Y111.389 E0.03096
G1 X108.97 Y111.21 E0.04137
G1 X108.755 Y110.987 E0.05168
G1 X108.581 Y110.714 E0.06245
G1 X108.472 Y110.435 E0.07241
G1 X108.416 Y110.125 E0.08289
G1 X108.422 Y109.811 E0.09333
G1 X108.488 Y109.51 E0.10358
G1 X108.611 Y109.227 E0.11384
G1 X108.79 Y108.97 E0.12426
G1 X109.013 Y108.755 E0.13456
G1 X109.286 Y108.581 E0.14533
G1 X109.565 Y108.472 E0.15529
G1 X109.875 Y108.416 E0.16577
G1 X110.189 Y108.422 E0.17622
G1 X110.49 Y108.488 E0.18647
G1 X110.773 Y108.611 E0.19673
G1 X111.03 Y108.79 E0.20715
G1 X111.245 Y109.013 E0.21745
G1 X111.419 Y109.286 E0.22822
G1 X111.528 Y109.565 E0.23818
G1 X111.584 Y109.875 E0.24866
G1 X111.578 Y110.189 E0.2591
G1 X111.513 Y110.488 E0.26928
G1 X111.389 Y110.773 E0.27962
G1 X111.21 Y111.03 E0.29003
G1 X110.987 Y111.245 E0.30034
G1 X110.714 Y111.419 E0.3111
G1 X110.435 Y111.528 E0.32107
G1 X110.125 Y111.584 E0.33154
;TIME_ELAPSED:5.535220
G1 F1500 E-6.16846
M140 S0
G91 ;Relative positioning
G1 E-2 F2700 ;Retract a bit
G1 E-2 Z0.2 F2400 ;Retract and raise Z
G1 X5 Y5 F3000 ;Wipe out
G1 Z10 ;Raise Z more
G90 ;Absolute positionning

G1 X0 Y220 ;Present print
M106 S0 ;Turn-off fan
M104 S0 ;Turn-off hotend
M140 S0 ;Turn-off bed

M84 X Y E ;Disable all steppers but Z

M82 ;absolute extrusion mode
M104 S0
;End of Gcode
;SETTING_3 {"global_quality": "[general]\\nversion = 4\\nname = Vias\\ndefinitio
;SETTING_3 n = creality_ender3pro\\n\\n[metadata]\\nquality_type = adaptive\\nty
;SETTING_3 pe = quality_changes\\nsetting_version = 16\\n\\n[values]\\nadhesion_
;SETTING_3 type = none\\nlayer_height = 0.5\\nmaterial_bed_temperature = 50\\nsu
;SETTING_3 pport_enable = False\\n\\n", "extruder_quality": ["[general]\\nversio
;SETTING_3 n = 4\\nname = Vias\\ndefinition = creality_ender3pro\\n\\n[metadata]
;SETTING_3 \\nposition = 0\\nquality_type = adaptive\\ntype = quality_changes\\n
;SETTING_3 intent_category = default\\nsetting_version = 16\\n\\n[values]\\nbott
;SETTING_3 om_layers = 0\\ncool_fan_speed = 10\\ninfill_pattern = grid\\ninfill_
;SETTING_3 sparse_density = 0\\nskirt_line_count = 1\\nspeed_print = 10.0\\ntop_
;SETTING_3 layers = 0\\nwall_thickness = 0.2\\nxy_offset = -0.2\\nxy_offset_laye
;SETTING_3 r_0 = -0.2\\n\\n"]}
