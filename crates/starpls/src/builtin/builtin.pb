
∂
apple»
multi_arch_platform=
+
platform_typeThe apple platform type.(apple_platform"ÒThe platform of the current configuration for the given platform type. This should only be invoked in a context where multiple architectures may be supported; consider <a href='#single_arch_platform'>single_arch_platform</a> for other cases.Ø
single_arch_cpustring"ìThe single "effective" architecture for this configuration (e.g., <code>i386</code> or <code>arm64</code>) in the context of rule logic that is only concerned with a single architecture (such as <code>objc_library</code>, which registers single-architecture compile actions).Ä
single_arch_platformapple_platform"◊The platform of the current configuration. This should only be invoked in a context where only a single architecture may be supported; consider <a href='#multi_arch_platform'>multi_arch_platform</a> for other cases.-A configuration fragment for Apple platforms.
t
bazel_androidc
"merge_android_manifest_permissionsbool"7The value of --merge_android_manifest_permissions flag.
µ
bazel_pyl
python_import_all_repositoriesbool"DThe value of the --experimental_python_import_all_repositories flag.;
python_pathstring"$The value of the --python_path flag.
Ú
coverage¢
output_generatorLabel"ÜReturns the label pointed to by the <a href="https://bazel.build/reference/command-line-reference#flag--coverage_output_generator"><code>--coverage_output_generator</code></a> option if coverage collection is enabled, otherwise returns <code>None</code>. Can be accessed with <a href="../globals/bzl.html#configuration_field"><code>configuration_field</code></a>:<br/><pre>attr.label(<br/>    default = configuration_field(<br/>        fragment = "coverage",<br/>        name = "output_generator"<br/>    )<br/>)</pre>AA configuration fragment representing the coverage configuration.
≈

cppU
apple_generate_dsymbool"8Whether to generate Apple debug symbol(.dSYM) artifacts.~
	conlyoptslist"kThe flags passed to Bazel by <a href="/docs/user-manual#flag--conlyopt"><code>--conlyopt</code></a> option.r
coptslist"cThe flags passed to Bazel by <a href="/docs/user-manual#flag--copt"><code>--copt</code></a> option.ç
custom_mallocLabel"ÙReturns label pointed to by <a href="/docs/user-manual#flag--custom_malloc"><code>--custom_malloc</code></a> option. Can be accessed with <a href="../globals/bzl.html#configuration_field"><code>configuration_field</code></a>:<br/><pre>attr.label(<br/>    default = configuration_field(<br/>        fragment = "cpp",<br/>        name = "custom_malloc"<br/>    )<br/>)</pre>x
cxxoptslist"gThe flags passed to Bazel by <a href="/docs/user-manual#flag--cxxopt"><code>--cxxopt</code></a> option.{
linkoptslist"iThe flags passed to Bazel by <a href="/docs/user-manual#flag--linkopt"><code>--linkopt</code></a> option.R
objc_generate_linkmapbool"3(Apple-only) Whether to generate linkmap artifacts.u
objc_should_strip_binarybool"S(Apple-only) whether to perform symbol and dead-code strippings on linked binaries.~
	objccoptslist"kThe flags passed to Bazel by <a href="/docs/user-manual#flag--objccopt"><code>--objccopt</code></a> option.!A configuration fragment for C++.
ç
j2objc]
translation_flagslist"BThe list of flags to be used when the j2objc compiler is invoked. $A configuration fragment for j2Objc.
¬
javaÙ
"bytecode_optimization_pass_actionsint"»This specifies the number of actions to divide the OPTIMIZATION stage of the bytecode optimizer into. Note that if split_bytecode_optimization_pass is set, this will only change behavior if it is > 2.O
bytecode_optimizer_mnemonicstring"(The mnemonic for the bytecode optimizer.E
default_javac_flagslist"(The default flags for the Java compiler.N
default_javac_flags_depsetdepset"(The default flags for the Java compiler._
default_jvm_optslist"EAdditional options to pass to the Java VM for each java_binary targetb
disallow_java_import_empty_jarsbool"7Returns true if empty java_import jars are not allowed.\
disallow_java_import_exportsbool"4Returns true if java_import exports are not allowed.b
multi_release_deploy_jarsbool"?The value of the --incompatible_multi_release_deploy_jars flag.f
one_version_enforcement_levelstring"=The value of the --experimental_one_version_enforcement flag.N
pluginslist"=A list containing the labels provided with --plugins, if any.^
run_android_lintbool"DThe value of the --experimental_run_android_lint_on_java_rules flag.å
 split_bytecode_optimization_passbool"bReturns whether the OPTIMIZATION stage of the bytecode optimizer will be split across two actions.C
strict_java_depsstring"'The value of the strict_java_deps flag.H
	use_ijarsbool"3Returns true iff Java compilation should use ijars.A java compiler configuration.
É
objcn
alwayslink_by_defaultbool"OReturns whether objc_library and objc_import should default to alwayslink=True.∫
coptslist"™Returns a list of options to use for compiling Objective-C.These options are applied after any default options but before options specified in the attributes of the rule.É
"copts_for_current_compilation_modelist"WReturns a list of default options to use for compiling Objective-C in the current mode.}
"disallow_sdk_frameworks_attributesbool"QReturns whether sdk_frameworks and weak_sdk_frameworks are disallowed attributes.@
generate_linkmapbool"&Whether to generate linkmap artifacts.j
ios_simulator_devicestring"JThe type of device (e.g. 'iPhone 6') to use when running on the simulator.r
ios_simulator_versionDottedVersion"JThe SDK version of the iOS simulator to use when running on the simulator.f
run_memleaksbool"PReturns a boolean indicating whether memleaks should be run during tests or not.k
should_strip_binarybool"NReturns whether to perform symbol and dead-code strippings on linked binaries.ì
signing_certificate_namestring"oReturns the flag-supplied certificate name to be used in signing, or None if no such certificate was specified.å
strip_executable_safelybool"kReturns whether executable strip action should use flag -x, which does not break dynamic symbol resolution.Å
uses_device_debug_entitlementsbool"YReturns whether device debug entitlements should be included when signing an application.)A configuration fragment for Objective-C.
ä
platform1
host_platformLabel"The current host platform.
platformLabel"The current target platformThe platform configuration.
@
proto7A configuration fragment representing protocol buffers.
¡
pyC
build_python_zipbool")The effective value of --build_python_zip_
default_python_versionstring"=The default python version from --incompatible_py3_is_defaultg
default_to_explicit_init_pybool"BThe value from the --incompatible_default_to_explicit_init_py flagM
disable_py2bool"8The value of the --incompatible_python_disable_py2 flag.a
disallow_native_rulesbool"BThe value of the --incompatible_python_disallow_native_rules flag.T
use_toolchainsbool"<The value from the --incompatible_use_python_toolchains flag$A configuration fragment for Python.
ö
AnalysisTestResultInfog
messagestring"TA descriptive message containing information about the test and its success/failure.t
successbool"cIf true, then the analysis-phase test represented by this target passed. If false, the test failed.†Encapsulates the result of analyis-phase testing. Build targets which return an instance of this provider signal to the build system that it should generate a 'stub' test executable which generates the equivalent test result. Analysis test rules (rules created with <code>analysis_test=True</code> <b>must</b> return an instance of this provider, and non-analysis-phase test rules <b>cannot</b> return this provider.
’
!AndroidNeverLinkLibrariesProviderÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.:Information about neverlink libraries for Android targets.
à
AppleDebugOutputsº
outputs_mapdict"¶A dictionary of: { arch: { output_type: file, output_type: file, ... } }, where 'arch' is any Apple architecture such as 'arm64' or 'armv7', 'output_type' is a string descriptor such as 'bitcode_symbols' or 'dsym_binary', and the file is the file matching that descriptor for that architecture.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.>A provider that holds debug outputs of an Apple binary target.
Á
AppleDynamicFramework†
binaryFile"èThe multi-architecture dylib binary of the dynamic framework. May be None if the rule providing the framework only specified framework imports.}
cc_infoCcInfo"jA provider which contains information about the transitive dependencies linked into the dynamic framework.v
framework_dirsdepset"\The framework path names used as link inputs in order to link against the dynamic framework.y
framework_filesdepset"^The full set of files that should be included as inputs to link against the dynamic framework.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.CA provider containing information about an Apple dynamic framework.
¡
AppleExecutableBinaryB
binaryFile"2The executable binary file output by apple_binary.r
cc_infoCcInfo"_A provider which contains information about the transitive dependencies linked into the binary.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.zA provider containing the executable binary output that was built using an apple_binary target with the 'executable' type.
¡
BaselineProfileProviderÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.0Baseline profile file used for Android binaries.
Ø
CcInfoV
compilation_contextCompilationContext"+Returns the <code>CompilationContext</code>J
linking_contextLinkingContext"'Returns the <code>LinkingContext</code>Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.äA provider for compilation and linking of C++. This is also a marking provider telling C++ rules that they can depend on the rule with this provider. If it is not intended for the rule to be depended on by C++, the rule should wrap the CcInfo in some other provider.
˙
CcToolchainConfigInfoS
protostring"BReturns CToolchain text proto from the CcToolchainConfigInfo data.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.ïAdditional layer of configurability for C++ rules. Encapsulates platform-dependent specifics of C++ actions through features and action configs. It is used to configure the C++ toolchain, and later on for command line construction. Replaces the functionality of CROSSTOOL file.
’
CcToolchainInfou
	all_filesNoneType"^Returns all toolchain files (so they can be passed to actions using this toolchain as inputs).5
ar_executableNoneType"The path to the ar binary.c
built_in_include_directoriesNoneType"9Returns the list of built-in directories of the compiler.#
compilerNoneType"C++ compiler.A
compiler_executableNoneType" The path to the compiler binary.1
cpuNoneType" Target CPU of the C++ toolchain.—
dynamic_runtime_libK
?
feature_configuration$Feature configuration to be queried.(NoneType"ÏReturns the files from `dynamic_runtime_lib` attribute (so they can be passed to actions using this toolchain as inputs). The caller can check whether the feature_configuration enables `static_link_cpp_runtimes` feature (if not, neither `static_runtime_lib` nor `dynamic_runtime_lib` have to be used), and use `static_runtime_lib` if static linking mode is active.9
gcov_executableNoneType"The path to the gcov binary.5
ld_executableNoneType"The path to the ld binary.&
libcNoneType"libc version string.Œ
needs_pic_for_dynamic_librariesK
?
feature_configuration$Feature configuration to be queried.(NoneType"›Returns true if this rule's compilations should apply -fPIC, false otherwise. Determines if we should apply -fPIC for this rule's C++ compilations depending on the C++ toolchain and presence of `--force_pic` Bazel option.5
nm_executableNoneType"The path to the nm binary.?
objcopy_executableNoneType"The path to the objcopy binary.?
objdump_executableNoneType"The path to the objdump binary.I
preprocessor_executableNoneType"$The path to the preprocessor binary.”
static_runtime_libK
?
feature_configuration$Feature configuration to be queried.(NoneType"ÔReturns the files from `static_runtime_lib` attribute (so they can be passed to actions using this toolchain as inputs). The caller should check whether the feature_configuration enables `static_link_cpp_runtimes` feature (if not, neither `static_runtime_lib` nor `dynamic_runtime_lib` should be used), and use `dynamic_runtime_lib` if dynamic linking mode is active.;
strip_executableNoneType"The path to the strip binary.”
sysrootNoneType"ΩReturns the sysroot to be used. If the toolchain compiler does not support different sysroots, or the sysroot is the same as the default sysroot, then this method returns <code>None</code>.8
target_gnu_system_nameNoneType"The GNU System Name..Information about the C++ compiler being used.
â
ConstraintCollectionProvides access to data about a collection of ConstraintValueInfo providers. <br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
⁄
ConstraintSettingInfoc
has_default_constraint_valuebool"=Whether there is a default constraint_value for this setting.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.ÂA specific constraint setting that may be used to define a platform. See <a href='/docs/platforms#defining-constraints-and-platforms'>Defining Constraints and Platforms</a> for more information.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
ˆ
ConstraintValueInfoÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.ËA value for a constraint setting that can be used to define a platform. See <a href='/docs/platforms#defining-constraints-and-platforms'>Defining Constraints and Platforms</a> for more information.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
á
DebugPackageInfoU
dwp_fileFile"CReturns the .dwp file (for fission builds) or null if --fission=no.S
stripped_fileFile"<Returns the stripped file (the explicit ".stripped" target).@
target_labelLabel")Returns the label for the *_binary targetÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.T
unstripped_fileFile";Returns the unstripped file (the default executable target)∏A provider for the binary file and its associated .dwp files, if fission is enabled.If Fission ({@url https://gcc.gnu.org/wiki/DebugFission}) is not enabled, the dwp file will be null.
ã 
DefaultInfo˘
data_runfilesrunfiles"›runfiles descriptor describing the files that this target needs when run in the condition that it is a <code>data</code> dependency attribute. Under most circumstances, use the <code>default_runfiles</code> parameter instead. See <a href='https://bazel.build/extending/rules#runfiles_features_to_avoid'>"runfiles features to avoid"</a> for details. £
default_runfilesrunfiles"Ñrunfiles descriptor describing the files that this target needs when run (via the <code>run</code> command or as a tool dependency).õ
filesdepset"âA <a href='../builtins/depset.html'><code>depset</code></a> of <a href='../builtins/File.html'><code>File</code></a> objects representing the default outputs to build when this target is specified on the bazel command line. By default it is all predeclared outputs.»
files_to_runFilesToRunProvider"£A <a href='../providers/FilesToRunProvider.html'><code>FilesToRunProvider</code></a> object containing information about the executable and runfiles of the target.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.˙A provider that gives general information about a target's direct and transitive files. Every rule type has this provider, even if it is not returned explicitly by the rule's implementation function. Each <code>DefaultInfo</code> instance has the following fields: <ul><li><code>files</code><li><code>files_to_run</code><li><code>data_runfiles</code><li><code>default_runfiles</code></ul>See the <a href='https://bazel.build/extending/rules'>rules</a> page for extensive guides on how to use this provider.
í
ExecutionInfoR

exec_groupstring"<The name of the exec group that is used to execute the test.c
requirementsdict"MA dict indicating special execution requirements, such as hardware platforms.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.RUse this provider to specify special environment requirements needed to run tests.
∆
FeatureFlagInfod
errorstring"SIf non-None, this error was generated when trying to compute current value of flag.û
is_valid_valueI
A
value6String, the value to check for validity for this flag.(bool"AThe value of the flag in the configuration used by the flag rule.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.n
valuestring"]The current value of the flag in the flag's current configuration. None if there is an error.FA provider used to access information about config_feature_flag rules.
;
file_provider*An interface for rules that provide files.
É
FilesToRunProviderE

executableFile"1The main executable or None if it does not exist.V
repo_mapping_manifestFile"7The repo mapping manifest or None if it does not exist.N
runfiles_manifestFile"3The runfiles manifest or None if it does not exist.
…
"GeneratedExtensionRegistryProviderÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.-Information about generated proto extensions.
ê
IncompatiblePlatformProviderÔA provider for targets that are incompatible with the target platform. See <a href='/docs/platforms#detecting-incompatible-targets-using-bazel-cquery'>Detecting incompatible targets using <code>bazel cquery</code></a> for more information.
™!
InstrumentedFilesInfoÊ
instrumented_filesdepset"«<a href="../builtins/depset.html"><code>depset</code></a> of <a href="../builtins/File.html"><code>File</code></a> objects representing instrumented source files for this target and its dependencies.ƒ
metadata_filesdepset"©<a href="../builtins/depset.html"><code>depset</code></a> of <a href="../builtins/File.html"><code>File</code></a> objects representing coverage metadata files for this target and its dependencies. These files contain additional information required to generate LCOV-format coverage output after the code is executed, e.g. the <code>.gcno</code> files generated when <code>gcc</code> is run with <code>-ftest-coverage</code>.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.ÍContains information about source files and instrumentation metadata files for rule targets matched by <a href="https://bazel.build/reference/command-line-reference#flag--instrumentation_filter"><code>--instrumentation_filter</code></a> for purposes of <a href="https://bazel.build/extending/rules#code_coverage">code coverage data collection</a>. When coverage data collection is enabled, a manifest containing the combined paths in <a href="#instrumented_files"><code>instrumented_files</code></a> and <a href="#metadata_files"><code>metadata_files</code></a> are passed to the test action as inputs, with the manifest's path noted in the environment variable <code>COVERAGE_MANIFEST</code>. The metadata files, but not the source files, are also passed to the test action as inputs. When <code>InstrumentedFilesInfo</code> is returned by an <a href="https://bazel.build/rules/aspects">aspect</a>'s implementation function, any <code>InstrumentedFilesInfo</code> from the base rule target is ignored.
‡
java_compilation_info<
boot_classpathlist"$Boot classpath for this Java target.L
compilation_classpathdepset"+Compilation classpath for this Java target.∑
javac_optionsdepset"ùA depset of options to java compiler. To get the exact list of options passed to javac in the correct order, use the tokenize_javacopts utility in rules_javaE
runtime_classpathdepset"(Run-time classpath for this Java target.:Provides access to compilation information for Java rules.
‚
java_output_jarsv
jarslist"hReturns information about outputs of this Java/Java-like target. Deprecated: Use java_info.java_outputs.¸
jdepsFile"ÏA manifest proto file. The protobuf file containing the manifest generated from JavaBuilder. This function returns a value when exactly one manifest proto file is present in the outputs.  Deprecated: Use java_info.java_outputs[i].jdeps.É
native_headersFile"ÍA jar containing CC header files supporting native method implementation.  This function returns a value when exactly one native headers jar file is present in the outputs. Deprecated: Use java_info.java_outputs[i].native_headers_jar.QInformation about outputs of a Java rule. Deprecated: use java_info.java_outputs.
Ω
JavaRuntimeInfo9
default_cdsFile"$Returns the JDK default CDS archive.7
filesdepset"&Returns the files in the Java runtime.`
hermetic_filesdepset"FReturns the files in the Java runtime needed for hermetic deployments.C
hermetic_static_libssequence"!Returns the JDK static libraries.Q
java_executable_exec_pathstring",Returns the execpath of the Java executable.Õ
java_executable_runfiles_pathstring"£Returns the path of the Java executable in runfiles trees. This should only be used when one needs to access the JVM during the execution of a binary or a test built by Bazel. In particular, when one needs to invoke the JVM during an action, java_executable_exec_path should be used instead.O
	java_homestring":Returns the execpath of the root of the Java installation.Ø
java_home_runfiles_pathstring"ãReturns the path of the Java installation in runfiles trees. This should only be used when one needs to access the JDK during the execution of a binary or a test built by Bazel. In particular, when one needs the JDK during an action, java_home should be used instead.0

lib_ct_symFile"Returns the lib/ct.sym file.2
lib_modulesFile"Returns the lib/modules file.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.]
versionint"MThe Java feature version of the runtime. This is 0 if the version is unknown..Information about the Java runtime being used.
ã
JavaToolchainInfok
bootclasspathdepset"RThe Java target bootclasspath entries. Corresponds to javac's -bootclasspath flag.R
ijarFilesToRunProvider"6A FilesToRunProvider representing the ijar executable.K
jacocorunnerFilesToRunProvider"'The jacocorunner used by the toolchain.>
java_runtimeJavaRuntimeInfo"The java runtime information.b
jvm_optdepset"OThe default options for the JVM running the java compiler and associated tools.$
labelLabel"The toolchain label.`
proguard_allowlisterFilesToRunProvider"4Return the binary to validate proguard configuration;

single_jarFilesToRunProvider"The SingleJar deploy jar.2
source_versionstring"The java source version.2
target_versionstring"The java target version.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.'
toolsdepset"The compilation tools.xProvides access to information about the Java toolchain rule. Accessible as a 'java_toolchain' field on a Target struct.
»
ObjcProvider∂
direct_module_mapssequence"ïModule map files from this target directly (no transitive module maps). Used to enforce proper use of private header files and for Swift compilation.ä
direct_sourcessequence"nAll direct source files from this target (no transitive files), including any headers in the 'srcs' attribute.[
j2objc_librarydepset"AStatic libraries that are built from J2ObjC-translated Java code.\

module_mapdepset"FClang module maps, used to enforce proper use of private header files..
sourcedepset"All transitive source files.¿
strict_includedepset"•Non-propagated include search paths specified with '-I' on the command line. Also known as header search paths (and distinct from <em>user</em> header search paths).í
umbrella_headerdepset"wClang umbrella header. Public headers are #included in umbrella headers to be compatible with J2ObjC segmented headers./A provider for compilation and linking of objc.
¡
OutputGroupInfoÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.∑A provider that indicates what output groups a rule has.<br>See <a href="https://bazel.build/extending/rules#requesting_output_files">Requesting output files</a> for more information.
Ë
PackageSpecificationInfoà
containsÃ
√
targetu<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>@A target which is checked if it exists inside the package group.(bool"-Checks if a target exists in a package group.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.KInformation about transitive package specifications used in package groups.
⁄
PlatformInfoÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.”Provides access to data about a specific platform. See <a href='/docs/platforms#defining-constraints-and-platforms'>Defining Constraints and Platforms</a> for more information.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
¿
ProguardSpecProviderÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.2Proguard specifications used for Android binaries.
æ
ProtoRegistryProviderÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead./Information about flavors for all built protos.
£
PyInfoj
has_py2_only_sourcesbool"LWhether any of this target's transitive sources requires a Python 2 runtime.j
has_py3_only_sourcesbool"LWhether any of this target's transitive sources requires a Python 3 runtime. 
importsdepset"∂A depset of import path strings to be added to the <code>PYTHONPATH</code> of executable Python targets. These are accumulated from the transitive <code>deps</code>.<p>The order of the depset is not guaranteed and may be changed in the future. It is recommended to use <code>default</code> order (the default).Ÿ
transitive_sourcesdepset"∫A (<code>postorder</code>-compatible) depset of <code>.py</code> files appearing in the target's <code>srcs</code> and the <code>srcs</code> of the target's transitive <code>deps</code>.ﬂ
uses_shared_librariesbool"øWhether any of this target's transitive <code>deps</code> has a shared library file (such as a <code>.so</code> file).<p>This field is currently unused in Bazel and may go away in the future.6Encapsulates information provided by the Python rules.
ø
PyRuntimeInfoŸ
bootstrap_templateFile"ºThe stub script template file to use. Should have %python_binary%, %workspace_name%, %main%, and %imports%. See @bazel_tools//tools/python:python_bootstrap_template.txt for more variables.ß
coverage_filesdepset"åThe files required at runtime for using <code>coverage_tool</code>. Will be <code>None</code> if no <code>coverage_tool</code> was provided.∫
coverage_toolFile"¢If set, this field is a <code>File</code> representing tool used for collecting code coverage information from python tests. Otherwise, this is <code>None</code>.â
filesdepset"˜If this is an in-build runtime, this field is a <code>depset</code> of <code>File</code>s that need to be added to the runfiles of an executable target that uses this runtime (in particular, files needed by <code>interpreter</code>). The value of <code>interpreter</code> need not be included in this field. If this is a platform runtime then this field is <code>None</code>.ê
interpreterFile"˙If this is an in-build runtime, this field is a <code>File</code> representing the interpreter. Otherwise, this is <code>None</code>. Note that an in-build runtime can use either a prebuilt, checked-in interpreter or an interpreter built from source.∑
interpreter_pathstring"öIf this is a platform runtime, this field is the absolute filesystem path to the interpreter on the target platform. Otherwise, this is <code>None</code>.û
python_versionstring"ÉIndicates whether this runtime uses Python major version 2 or 3. Valid values are (only) <code>"PY2"</code> and <code>"PY3"</code>.Ø
stub_shebangstring"ñ"Shebang" expression prepended to the bootstrapping Python stub script used when executing <code>py_binary</code> targets.  Does not apply to Windows.æContains information about a Python runtime, as returned by the <code>py_runtime</code>rule.<p>A Python runtime describes either a <em>platform runtime</em> or an <em>in-build runtime</em>. A platform runtime accesses a system-installed interpreter at a known path, whereas an in-build runtime points to a <code>File</code> that acts as the interpreter. In both cases, an "interpreter" is really any executable binary or wrapper script that is capable of running a Python script passed on the command line, following the same conventions as the standard CPython interpreter.
Ø
RunEnvironmentInfoÍ
environmentdict"‘A map of string keys and values that represent environment variables and their values. These will be made available when the target that returns this provider is executed, either as a test or via the run command.º
inherited_environmentlist"úA sequence of names of environment variables. These variables are made  available with their current value taken from the shell environment when the target that returns this provider is executed, either as a test or via the run command. If a variable is contained in both <code>environment</code> and <code>inherited_environment</code>, the value inherited from the shell environment will take precedence if set.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.wA provider that can be returned from executable rules to control the environment in which their executable is executed.
˝
TemplateVariableInfoÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.w
	variablesdict"dReturns the make variables defined by this target as a dictionary with string keys and string valuesıEncapsulates template variables, that is, variables that can be referenced by strings like <code>$(VARIABLE)</code> in BUILD files and expanded by <code>ctx.expand_make_variables</code> and implicitly in certain attributes of built-in rules.</p><p><code>TemplateVariableInfo</code> can be created by calling its eponymous constructor with a string-to-string dict as an argument that specifies the variables provided.</p><p>Example: <code>platform_common.TemplateVariableInfo({'FOO': 'bar'})</code></p>
†
ToolchainInfoÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.òProvider returned by <a href="/docs/toolchains#defining-toolchains">toolchain rules</a> to share data with <a href="/docs/toolchains#writing-rules-that-use-toolchains">rules which depend on toolchains</a>. Read about <a href='/docs/toolchains'>toolchains</a> for more information.
≤
ToolchainTypeInfoÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.H

type_labelLabel"3The label uniquely identifying this toolchain type.‹Provides access to data about a specific toolchain type. <br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>
¿
XcodePropertiesz
default_ios_sdk_versionstring"WThe default iOS sdk version for this version of xcode, or <code>None</code> if unknown.~
default_macos_sdk_versionstring"YThe default macOS sdk version for this version of xcode, or <code>None</code> if unknown.|
default_tvos_sdk_versionstring"XThe default tvOS sdk version for this version of xcode, or <code>None</code> if unknown.Ñ
default_visionos_sdk_versionstring"\The default visionOS sdk version for this version of xcode, or <code>None</code> if unknown.Ç
default_watchos_sdk_versionstring"[The default watchOS sdk version for this version of xcode, or <code>None</code> if unknown.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.a
xcode_versionstring"HThe xcode version, or <code>None</code> if the xcode version is unknown.NA provider containing information about a version of Xcode and its properties.
à
XcodeVersionConfig®
availabilitystring"çReturns the availability of this Xcode version, 'remote' if the version is only available remotely, 'local' if the version is only available locally, 'both' if the version is available both locally and remotely, or 'unknown' if the availability could not be determined.d
execution_infodict"JReturns the execution requirements for actions that use this Xcode config.¿
minimum_os_for_platform_type<
+
platform_typeThe apple platform type.(DottedVersion"bThe minimum compatible OS version for target simulator and devices for a particular platform type.™
sdk_version_for_platform2
!
platformThe apple platform.(DottedVersion"ZThe version of the platform SDK that will be used to build targets for the given platform.Â	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.°
xcode_versionDottedVersion"Returns the Xcode version that is being used to build.<p>This will return <code>None</code> if no Xcode versions are available.WThe set of Apple versions computed from command line options and the xcode_config rule.
¢
Action—
argssequence"æA list of frozen <a href="../builtins/Args.html">Args</a> objects containing information about the action arguments. These objects contain accurate argument information, including arguments involving expanded action output directories. However, <a href="../builtins/Args.html">Args</a> objects are not readable in the analysis phase. For a less accurate account of arguments which is available in the analysis phase, see <a href="#argv">argv</a>. <p>Note that some types of actions do not yet support exposure of this field. For such action types, this is <code>None</code>.›
argvsequence" For actions created by <a href="../builtins/actions.html#run">ctx.actions.run()</a> or <a href="../builtins/actions.html#run_shell">ctx.actions.run_shell()</a>  an immutable list of the arguments for the command line to be executed. Note that for shell actions the first two arguments will be the shell path and <code>"-c"</code>.Ó
contentstring"⁄For actions created by <a href="../builtins/actions.html#write">ctx.actions.write()</a> or <a href="../builtins/actions.html#expand_template">ctx.actions.expand_template()</a>, the contents of the file to be written, if those contents can be computed during  the analysis phase. The value is <code>None</code> if the contents cannot be determined until the execution phase, such as when a directory in an <a href="../builtins/Args.html">Args</a> object needs to be expanded.Ë
envdict"⁄The 'fixed' environment variables for this action. This includes only environment settings which are explicitly set by the action definition, and thus omits settings which are only pre-set in the execution environment.:
inputsdepset"(A set of the input files of this action.1
mnemonicstring"The mnemonic for this action.<
outputsdepset")A set of the output files of this action.∏
substitutionsdict"†For actions created by <a href="../builtins/actions.html#expand_template">ctx.actions.expand_template()</a>, an immutable dict holding the substitution mapping.üAn action created during rule analysis.<p>This object is visible for the purpose of testing, and may be obtained from an <code>Actions</code> provider. It is normally not necessary to access <code>Action</code> objects or their fields within a rule's implementation function. You may instead want to see the <a href='https://bazel.build/extending/rules#actions'>Rules page</a> for a general discussion of how to use actions when defining custom rules, or the <a href='../builtins/actions.html'>API reference</a> for creating actions.<p>Some fields of this object are only applicable for certain kinds of actions. Fields that are inapplicable are set to <code>None</code>.
≈¶
actions`
argsArgs"PReturns an Args object that can be used to build memory-efficient command lines.–
declare_directoryÉ
¶
filenameóIf no 'sibling' provided, path of the new directory, relative to the current package. Otherwise a base name for a file ('sibling' defines a directory).(
—
siblingM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>qA file that lives in the same directory as the newly declared directory. The file must be in the current package."NoneFile"¥Declares that the rule or aspect creates a directory with the given name, in the current package. You must create an action that generates the directory. The contents of the directory are not directly accessible from Starlark, but can be expanded in an action command with <a href="../builtins/Args.html#add_all"><code>Args.add_all()</code></a>. Only regular files and directories can be in the expanded contents of a declare_directory.™

declare_file˚
§
filenameïIf no 'sibling' provided, path of the new file, relative to the current package. Otherwise a base name for a file ('sibling' determines a directory).(
À
siblingM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>kA file that lives in the same directory as the newly created file. The file must be in the current package."NoneFile"õDeclares that the rule or aspect creates a file with the given filename. If <code>sibling</code> is not specified, the file name is relative to the package directory, otherwise the file is in the same directory as <code>sibling</code>. Files cannot be created outside of the current package.<p>Remember that in addition to declaring a file, you must separately create an action that emits the file. Creating that action will require passing the returned <code>File</code> object to the action's construction function.<p>Note that <a href='https://bazel.build/extending/rules#files'>predeclared output files</a> do not need to be (and cannot be) declared using this function. You can obtain their <code>File</code> objects from <a href="../builtins/ctx.html#outputs"><code>ctx.outputs</code></a> instead. <a href="https://github.com/bazelbuild/examples/tree/main/rules/computed_dependencies/hash.bzl">See example of use</a>.®
declare_symlink÷
§
filenameïIf no 'sibling' provided, path of the new symlink, relative to the current package. Otherwise a base name for a file ('sibling' defines a directory).(
¶
siblingM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>FA file that lives in the same directory as the newly declared symlink."NoneFile"ªDeclares that the rule or aspect creates a symlink with the given name in the current package. You must create an action that generates this symlink. Bazel will never dereference this symlink and will transfer it verbatim to sandboxes or remote executors. Symlinks inside tree artifacts are not currently supported.„

do_nothingœ
V
mnemonicHA one-word description of the action, for example, CppCompile or GoLink.(
Í
inputs≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>&List of the input files of the action."[]NoneType"ÇCreates an empty action that neither executes a command nor produces any output, but that is useful for inserting 'extra actions'.‘
expand_template√
D
template6The template file, which is a UTF-8 encoded text file.(
@
output4The output file, which is a UTF-8 encoded text file.(
G
substitutions2Substitutions to make when expanding the template."{}
E
is_executable-Whether the output file should be executable."False
û
computed_substitutionsG<a class="anchor" href="../builtins/TemplateDict.html">TemplateDict</a>2Substitutions to make when expanding the template."unboundNoneType"˙Creates a template expansion action. When the action is executed, it will generate a file based on a template. Parts of the template will be replaced using the <code>substitutions</code> dictionary, in the order the substitutions are specified. Whenever a key of the dictionary appears in the template (or a result of a previous substitution), it is replaced with the associated value. There is no special syntax for the keys. You may, for example, use curly braces to avoid conflicts (for example, <code>{KEY}</code>). <a href="https://github.com/bazelbuild/examples/blob/main/rules/expand_template/hello.bzl">See example of use</a>.À3
run•2
©
outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s'List of the output files of the action.(
Ù
inputs≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>0List or depset of the input files of the action."[]
ó
unused_inputs_listM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>´File containing list of inputs unused by the action. <p>The content of this file (generally one of the outputs of the action) corresponds to  the list of input files that were not used during the whole action execution. Any change in those files must not affect in any way the outputs of the action."None
é

executableÃ<a class="anchor" href="../builtins/File.html">File</a>; or <a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../providers/FilesToRunProvider.html">FilesToRunProvider</a>/The executable file to be called by the action.(
ú
toolsw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>êList or depset of any tools needed by the action. Tools are inputs with additional runfiles that are automatically made available to the action. When a list is provided, it can be a heterogenous collection of Files, FilesToRunProvider instances, or depsets of Files. Files which are directly in the list and come from ctx.executable will have their runfiles automatically added. When a depset is provided, it must contain only Files. In both cases, files within depsets are not cross-referenced with ctx.executable for runfiles."unbound
≈
	arguments7<a class="anchor" href="../core/list.html">sequence</a>{Command line arguments of the action. Must be a list of strings or <a href="#args"><code>actions.args()</code></a> objects."[]
©
mnemonicM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>HA one-word description of the action, for example, CppCompile or GoLink."None
Í
progress_messageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ÄProgress message to show to the user during the build, for example, "Compiling foo.cc to create foo.o". The message may contain <code>%{label}</code>, <code>%{input}</code>, or <code>%{output}</code> patterns, which are substituted with label string, first input, or output's path, respectively. Prefer to use patterns instead of static strings, because the former are more efficient."None
¶
use_default_shell_envÖWhether the action should use the default shell environment, which consists of a few OS-dependent variables as well as variables set via <a href="/reference/command-line-reference#flag--action_env"><code>--action_env</code></a>.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment if <a href="/reference/command-line-reference#flag--incompatible_merge_fixed_and_default_shell_env"><code>--incompatible_merge_fixed_and_default_shell_env</code></a> is enabled (default). If the flag is not enabled, <code>env</code> is ignored."False
ß
envI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>ŒSets the dictionary of environment variables.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment if <a href="/reference/command-line-reference#flag--incompatible_merge_fixed_and_default_shell_env"><code>--incompatible_merge_fixed_and_default_shell_env</code></a> is enabled (default). If the flag is not enabled, <code>env</code> is ignored."None
∆
execution_requirementsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>[Information for scheduling the action. See <a href="#common.tags">tags</a> for useful keys."None
…
input_manifestsM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>a(Experimental) sets the input runfiles metadata; they are typically generated by resolve_command."None
◊

exec_groupM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>tRuns the action on the given exec group's execution platform. If none, uses the target's default execution platform."None
È
shadowed_action;<a class="anchor" href="../builtins/Action.html">Action</a>íRuns the action using the given shadowed action's inputs and environment added to the action's inputs list and environment. The action environment can overwrite any of the shadowed action's environment variables. If none, uses only the action's inputs and given environment."None
°
resource_setcallable; or <code>None</code>ÍA callback function that returns a resource set dictionary, used to estimate resource usage at execution time if this action is run locally.<p>The function accepts two positional arguments: a string representing an OS name (e.g. "osx"), and an integer representing the number of inputs to the action. The returned dictionary may contain the following entries, each of which may be a float or an int:<ul><li>"cpu": number of CPUs; default 1<li>"memory": in MB; default 250<li>"local_test": number of local tests; default 1</ul><p>If this parameter is set to <code>None</code> or if <code>--experimental_action_resource_set</code> is false, the default values are used.<p>The callback must be top-level (lambda and nested functions aren't allowed)."None
¯
	toolchainã<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>”<p>Toolchain type of the executable or tools used in this action.</p><p>If executable and tools are not coming from a toolchain, set this parameter to `None`.</p><p>If executable and tools are coming from a toolchain, toolchain type must be set so that the action executes on the correct execution platform.</p><p>Note that the rule which creates this action needs to define this toolchain inside its 'rule()' function.</p><p>When `toolchain` and `exec_group` parameters are both set, `exec_group` will be used. An error is raised in case the `exec_group` doesn't specify the same toolchain.</p>"unboundNoneType"õCreates an action that runs an executable. <a href="https://github.com/bazelbuild/examples/tree/main/rules/actions_run/execute.bzl">See example of use</a>.†7
	run_shellÚ5
©
outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s'List of the output files of the action.(
Ù
inputs≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>0List or depset of the input files of the action."[]
ï
tools≥<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <a class="anchor" href="../builtins/depset.html">depset</a>ÃList or depset of any tools needed by the action. Tools are inputs with additional runfiles that are automatically made available to the action. The list can contain Files or FilesToRunProvider instances."unbound
â
	arguments7<a class="anchor" href="../core/list.html">sequence</a>æCommand line arguments of the action. Must be a list of strings or <a href="#args"><code>actions.args()</code></a> objects.<p>Bazel passes the elements in this attribute as arguments to the command.The command can access these arguments using shell variable substitutions such as <code>$1</code>, <code>$2</code>, etc. Note that since Args objects are flattened before indexing, if there is an Args object of unknown size then all subsequent strings will be at unpredictable indices. It may be useful to use <code>$@</code> (to retrieve all arguments) in conjunction with Args objects of indeterminate size.<p>In the case where <code>command</code> is a list of strings, this parameter may not be used."[]
©
mnemonicM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>HA one-word description of the action, for example, CppCompile or GoLink."None
¢
commandØ<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s‚
Shell command to execute. This may either be a string (preferred) or a sequence of strings <b>(deprecated)</b>.<p>If <code>command</code> is a string, then it is executed as if by <code>sh -c &lt;command&gt; "" &lt;arguments&gt;</code> -- that is, the elements in <code>arguments</code> are made available to the command as <code>$1</code>, <code>$2</code> (or <code>%1</code>, <code>%2</code> if using Windows batch), etc. If <code>arguments</code> contains any <a href="#args"><code>actions.args()</code></a> objects, their contents are appended one by one to the command line, so <code>$</code><i>i</i> can refer to individual strings within an Args object. Note that if an Args object of unknown size is passed as part of <code>arguments</code>, then the strings will be at unknown indices; in this case the <code>$@</code> shell substitution (retrieve all arguments) may be useful.<p><b>(Deprecated)</b> If <code>command</code> is a sequence of strings, the first item is the executable to run and the remaining items are its arguments. If this form is used, the <code>arguments</code> parameter must not be supplied. <i>Note that this form is deprecated and will soon be removed. It is disabled with `--incompatible_run_shell_command_string`. Use this flag to verify your code is compatible. </i><p>Bazel uses the same shell to execute the command as it does for genrules.(
Í
progress_messageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ÄProgress message to show to the user during the build, for example, "Compiling foo.cc to create foo.o". The message may contain <code>%{label}</code>, <code>%{input}</code>, or <code>%{output}</code> patterns, which are substituted with label string, first input, or output's path, respectively. Prefer to use patterns instead of static strings, because the former are more efficient."None
¶
use_default_shell_envÖWhether the action should use the default shell environment, which consists of a few OS-dependent variables as well as variables set via <a href="/reference/command-line-reference#flag--action_env"><code>--action_env</code></a>.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment if <a href="/reference/command-line-reference#flag--incompatible_merge_fixed_and_default_shell_env"><code>--incompatible_merge_fixed_and_default_shell_env</code></a> is enabled (default). If the flag is not enabled, <code>env</code> is ignored."False
ß
envI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>ŒSets the dictionary of environment variables.<p>If both <code>use_default_shell_env</code> and <code>env</code> are set to <code>True</code>, values set in <code>env</code> will overwrite the default shell environment if <a href="/reference/command-line-reference#flag--incompatible_merge_fixed_and_default_shell_env"><code>--incompatible_merge_fixed_and_default_shell_env</code></a> is enabled (default). If the flag is not enabled, <code>env</code> is ignored."None
∆
execution_requirementsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>[Information for scheduling the action. See <a href="#common.tags">tags</a> for useful keys."None
…
input_manifestsM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>a(Experimental) sets the input runfiles metadata; they are typically generated by resolve_command."None
◊

exec_groupM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>tRuns the action on the given exec group's execution platform. If none, uses the target's default execution platform."None
Â
shadowed_action;<a class="anchor" href="../builtins/Action.html">Action</a>éRuns the action using the given shadowed action's discovered inputs added to the action's inputs list. If none, uses only the action's inputs."None
ª
resource_setcallable; or <code>None</code>ÑA callback function for estimating resource usage if run locally. See<a href="#run.resource_set"><code>ctx.actions.run()</code></a>."None
¯
	toolchainã<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>”<p>Toolchain type of the executable or tools used in this action.</p><p>If executable and tools are not coming from a toolchain, set this parameter to `None`.</p><p>If executable and tools are coming from a toolchain, toolchain type must be set so that the action executes on the correct execution platform.</p><p>Note that the rule which creates this action needs to define this toolchain inside its 'rule()' function.</p><p>When `toolchain` and `exec_group` parameters are both set, `exec_group` will be used. An error is raised in case the `exec_group` doesn't specify the same toolchain.</p>"unboundNoneType"ùCreates an action that runs a shell command. <a href="https://github.com/bazelbuild/examples/tree/main/rules/shell_command/rules.bzl">See example of use</a>.⁄
symlink∂
&
outputThe output of this action.(
ì
target_fileM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>/The File that the output symlink will point to."None
 
target_pathM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>fThe exact path that the output symlink will point to. No normalization or other processing is applied."None
¸
is_executable„May only be used with <code>target_file</code>, not <code>target_path</code>. If true, when the action is executed, the <code>target_file</code>'s path is checked to confirm that it is executable, and an error is reported if it is not. Setting <code>is_executable</code> to False does not mean the target is not executable, just that no verification is done.<p>This feature does not make sense for <code>target_path</code> because dangling symlinks might not exist at build time.</p>"False
ü
progress_messageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>6Progress message to show to the user during the build."NoneNoneType"ïCreates an action that writes a symlink in the file system.<p>This function must be called with exactly one of <code>target_file</code> or <code>target_path</code> specified.</p><p>When you use <code>target_file</code>, declare <code>output</code> with <a href="#declare_file"><code>declare_file()</code></a> or <a href="#declare_directory"><code>declare_directory()</code></a> and match the type of <code>target_file</code>. This makes the symlink point to <code>target_file</code>. Bazel invalidates the output of this action whenever the target of the symlink or its contents change.</p><p>Otherwise, when you use <code>target_path</code>, declare <code>output</code> with <a href="#declare_symlink"><code>declare_symlink()</code></a>). In this case, the symlink points to <code>target_path</code>. Bazel never resolves the symlink and the output of this action is invalidated only when the text contents of the symlink (that is, the value of <code>readlink()</code>) changes. In particular, this can be used to create a dangling symlink.</p>g
template_dictTemplateDict"FReturns a TemplateDict object for memory-efficient template expansion.™
write‰

outputThe output file.(
Ú
contents<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Args.html">Args</a>pthe contents of the file. May be a either a string or an <a href="#args"><code>actions.args()</code></a> object.(
E
is_executable-Whether the output file should be executable."FalseNoneType"πCreates a file write action. When the action is executed, it will write the given content to a file. This is used to generate files using information available in the analysis phase. If the file is large and with a lot of static content, consider using <a href="#expand_template"><code>expand_template</code></a>.ãModule providing functions to create actions. Access this module using <a href="../builtins/ctx.html#actions"><code>ctx.actions</code></a>.
Ø
apple_platformá
	is_devicebool"tReturns <code>True</code> if this platform is a device platform or <code>False</code> if it is a simulator platform.Ò
name_in_pliststring"◊The name of the platform as it appears in the <code>CFBundleSupportedPlatforms</code> entry of an Info.plist file and in Xcode's platforms directory, without the extension (for example, <code>iPhoneOS</code> or <code>iPhoneSimulator</code>).<br>This name, when converted to lowercase (e.g., <code>iphoneos</code>, <code>iphonesimulator</code>), can be passed to Xcode's command-line tools like <code>ibtool</code> and <code>actool</code> when they expect a platform name.Q
platform_typeapple_platform_type"+Returns the platform type of this platform.À	Corresponds to Xcode's notion of a platform as would be found in <code>Xcode.app/Contents/Developer/Platforms</code>. Each platform represents an Apple platform type (such as iOS or tvOS) combined with one or more related CPU architectures. For example, the iOS simulator platform supports <code>x86_64</code> and <code>i386</code> architectures.<p>Specific instances of this type can be retrieved from the fields of the <a href='../toplevel/apple_common.html#platform'>apple_common.platform</a> struct:<br><ul><li><code>apple_common.platform.ios_device</code></li><li><code>apple_common.platform.ios_simulator</code></li><li><code>apple_common.platform.macos</code></li><li><code>apple_common.platform.tvos_device</code></li><li><code>apple_common.platform.tvos_simulator</code></li><li><code>apple_common.platform.watchos_device</code></li><li><code>apple_common.platform.watchos_simulator</code></li></ul><p>More commonly, however, the <a href='../fragments/apple.html'>apple</a> configuration fragment has fields/methods that allow rules to determine the platform for which a target is being built.<p>Example:<br><pre class='language-python'>
p = apple_common.platform.ios_device
print(p.name_in_plist)  # 'iPhoneOS'
</pre>
Å
apple_platform_typeÈDescribes an Apple "platform type", such as iOS, macOS, tvOS, visionOS, or watchOS. This is distinct from a "platform", which is the platform type combined with one or more CPU architectures.<p>Specific instances of this type can be retrieved by accessing the fields of the <a href='../toplevel/apple_common.html#platform_type'>apple_common.platform_type</a>:<br><ul><li><code>apple_common.platform_type.ios</code></li><li><code>apple_common.platform_type.macos</code></li><li><code>apple_common.platform_type.tvos</code></li><li><code>apple_common.platform_type.watchos</code></li></ul><p>Likewise, the platform type of an existing platform value can be retrieved using its <code>platform_type</code> field.<p>Platform types can be converted to a lowercase string (e.g., <code>ios</code> or <code>macos</code>) using the <a href='../globals/all.html#str'>str</a> function.
Ã
apple_toolchaine
developer_dirstring"JReturns the Developer directory inside of Xcode for a given configuration.∏
 platform_developer_framework_dir>
4
configuration!The apple configuration fragment.(string"TReturns the platform frameworks directory inside of Xcode for a given configuration.^
sdk_dirstring"IReturns the platform directory inside of Xcode for a given configuration.7Utilities for resolving items from the Apple toolchain.
∞|
Args¿
addç
Ñ
arg_name_or_valueÏIf two positional parameters are passed this is interpreted as the arg name. The arg name is added before the value without any processing. If only one positional parameter is passed, it is interpreted as <code>value</code> (see below).(
√
value∞The object to append. It will be converted to a string using the standard conversion mentioned above. Since there is no <code>map_each</code> parameter for this function, <code>value</code> should be either a string or a <code>File</code>. A list, tuple, depset, or directory <code>File</code> must be passed to <a href='#add_all'><code>add_all()</code> or <a href='#add_joined'><code>add_joined()</code></a> instead of this method."unbound
∑
formatM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>XA format string pattern, to be applied to the stringified version of <code>value</code>."NoneArgs")Appends an argument to this command line.÷.
add_allˇ#
Ó
arg_name_or_values’If two positional parameters are passed this is interpreted as the arg name. The arg name is added before the <code>values</code> as a separate argument without any processing. This arg name will not be added if <code>omit_if_empty</code> is true (the default) and no other items are appended (as happens if <code>values</code> is empty or all of its items are filtered). If only one positional parameter is passed, it is interpreted as <code>values</code> (see below).(
ƒ
valuesw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>8The list, tuple, or depset whose items will be appended."unbound
“
map_eachcallable; or <code>None</code>üA function that converts each item to zero or more strings, which may be further processed before appending. If this param is not provided, the standard conversion is used.<p>The function is passed either one or two positional arguments: the item to convert, followed by an optional <a href='../builtins/DirectoryExpander.html'><code>DirectoryExpander</code></a>. The second argument will be passed only if the supplied function is user-defined (not built-in) and declares more than one parameter.<p>The return value's type depends on how many arguments are to be produced for the item:<ul><li>In the common case when each item turns into one string, the function     should return that string.<li>If the item is to be filtered out entirely, the function should return     <code>None</code>.<li>If the item turns into multiple strings, the function returns a list of     those strings.</ul>Returning a single string or <code>None</code> has the same effect as returning a list of length 1 or length 0 respectively. However, it is more efficient and readable to avoid creating a list where it is not needed.<p>Ordinarily, items that are directories are automatically expanded to their contents when <code>expand_directories=True</code> is set. However, this will not expand directories contained inside other values -- for instance, when the items are structs that have directories as fields. In this situation, the <code>DirectoryExpander</code> argument can be applied to manually obtain the files of a given directory.<p>To avoid unintended retention of large analysis-phase data structures into the execution phase, the <code>map_each</code> function must be declared by a top-level <code>def</code> statement; it may not be a nested function closure by default.<p><i>Warning:</i> <a href='../globals/all.html#print'><code>print()</code></a> statements that are executed during the call to <code>map_each</code> will not produce any visible output."None
à
format_eachM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>£An optional format string pattern, applied to each string returned by the <code>map_each</code> function. The format string must have exactly one '%s' placeholder."None
≈
before_eachM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>aAn optional argument to append before each argument derived from <code>values</code> is appended."None
—
omit_if_emptyπIf true, if there are no arguments derived from <code>values</code> to be appended, then all further processing is suppressed and the command line will be unchanged. If false, the arg name and <code>terminate_with</code>, if provided, will still be appended regardless of whether or not there are other arguments."True
«
uniquify≥If true, duplicate arguments that are derived from <code>values</code> will be omitted. Only the first occurrence of each argument will remain. Usually this feature is not needed because depsets already omit duplicates, but it can be useful if <code>map_each</code> emits the same string for multiple items."False
¨
expand_directoriesèIf true, any directories in <code>values</code> will be expanded to a flat list of files. This happens before <code>map_each</code> is applied."True
‰
terminate_withM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>¸An optional argument to append after all other arguments. This argument will not be added if <code>omit_if_empty</code> is true (the default) and no other items are appended (as happens if <code>values</code> is empty or all of its items are filtered)."None
‚
allow_closure…If true, allows the use of closures in function parameters like <code>map_each</code>. Usually this isn't necessary and it risks retaining large analysis-phase data structures into the execution phase."FalseArgs"»
Appends multiple arguments to this command line. The items are processed lazily during the execution phase.<p>Most of the processing occurs over a list of arguments to be appended, as per the following steps:<ol><li>Each directory <code>File</code> item is replaced by all <code>File</code>s recursively contained in that directory.</li><li>If <code>map_each</code> is given, it is applied to each item, and the     resulting lists of strings are concatenated to form the initial argument     list. Otherwise, the initial argument list is the result of applying the     standard conversion to each item.<li>Each argument in the list is formatted with <code>format_each</code>, if     present.<li>If <code>uniquify</code> is true, duplicate arguments are removed. The first     occurrence is the one that remains.<li>If a <code>before_each</code> string is given, it is inserted as a new     argument before each existing argument in the list. This effectively doubles     the number of arguments to be appended by this point.<li>Except in the case that the list is empty and <code>omit_if_empty</code> is     true (the default), the arg name and <code>terminate_with</code> are     inserted as the first and last arguments, respectively, if they are given.</ol>Note that empty strings are valid arguments that are subject to all these processing steps.§

add_joined¯
ˇ
arg_name_or_valuesÊIf two positional parameters are passed this is interpreted as the arg name. The arg name is added before <code>values</code> without any processing. This arg will not be added if <code>omit_if_empty</code> is true (the default) and there are no strings derived from <code>values</code> to join together (which can happen if <code>values</code> is empty or all of its items are filtered). If only one positional parameter is passed, it is interpreted as <code>values</code> (see below).(
¬
valuesw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>6The list, tuple, or depset whose items will be joined."unbound
Ë
	join_withÿA delimiter string used to join together the strings obtained from applying <code>map_each</code> and <code>format_each</code>, in the same manner as <a href='../core/string.html#join'><code>string.join()</code></a>.(
s
map_eachcallable; or <code>None</code>ASame as for <a href='#add_all.map_each'><code>add_all</code></a>."None
®
format_eachM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>DSame as for <a href='#add_all.format_each'><code>add_all</code></a>."None
ﬂ
format_joinedM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>yAn optional format string pattern applied to the joined string. The format string must have exactly one '%s' placeholder."None
ú
omit_if_emptyÑIf true, if there are no strings to join together (either because <code>values</code> is empty or all its items are filtered), then all further processing is suppressed and the command line will be unchanged. If false, then even if there are no strings to join together, two arguments will be appended: the arg name followed by an empty string (which is the logical join of zero strings)."True
T
uniquifyASame as for <a href='#add_all.uniquify'><code>add_all</code></a>."False
g
expand_directoriesKSame as for <a href='#add_all.expand_directories'><code>add_all</code></a>."True
^
allow_closureFSame as for <a href='#add_all.allow_closure'><code>add_all</code></a>."FalseArgs"öAppends an argument to this command line by concatenating together multiple values using a separator. The items are processed lazily during the execution phase.<p>Processing is similar to <a href='#add_all'><code>add_all()</code></a>, but the list of arguments derived from <code>values</code> is combined into a single argument as if by <code>join_with.join(...)</code>, and then formatted using the given <code>format_joined</code> string template. Unlike <code>add_all()</code>, there is no <code>before_each</code> or <code>terminate_with</code> parameter since these are not generally useful when the items are combined into a single argument.<p>If after filtering there are no strings to join into an argument, and if <code>omit_if_empty</code> is true (the default), no processing is done. Otherwise if there are no strings to join but <code>omit_if_empty</code> is false, the joined string will be an empty string.Ä
set_param_file_format≥
™
formatùMust be one of:<ul><li>"multiline": Each item (argument name or value) is written verbatim to the param file with a newline character following it.</li><li>"shell": Same as "multiline", but the items are shell-quoted</li><li>"flag_per_line": Same as "multiline", but (1) only flags (beginning with '--') are written to the param file, and (2) the values of the flags, if any, are written on the same line with a '=' separator. This is the format expected by the Abseil flags library.</li></ul><p>The format defaults to "shell" if not called.(Args"1Sets the format of the param file, if one is usedÀ
use_param_file≥
ı
param_file_arg‡A format string with a single "%s". If the args are spilled to a params file then they are replaced with an argument consisting of this string formatted with the path of the params file.<p>For example, if the args are spilled to a params file "params.txt", then specifying "--file=%s" would cause the action command line to contain "--file=params.txt".(
≤

use_alwaysúWhether to always spill the args to a params file. If false, bazel will decide whether the arguments need to be spilled based on your system and arg length."FalseArgs"ÇSpills the args to a params file, replacing them with a pointer to the param file. Use when your args may be too large for the system's command length limits.<p>Bazel may choose to elide writing the params file to the output tree during execution for efficiency. If you are debugging actions and want to inspect the param file, pass <code>--materialize_param_files</code> to your build.ì!An object that encapsulates, in a memory-efficient way, the data needed to build part or all of a command line.<p>It often happens that an action requires a large command line containing values accumulated from transitive dependencies. For example, a linker command line might list every object file needed by all of the libraries being linked. It is best practice to store such transitive data in <a href='../builtins/depset.html'><code>depset</code></a>s, so that they can be shared by multiple targets. However, if the rule author had to convert these depsets into lists of strings in order to construct an action command line, it would defeat this memory-sharing optimization.<p>For this reason, the action-constructing functions accept <code>Args</code> objects in addition to strings. Each <code>Args</code> object represents a concatenation of strings and depsets, with optional transformations for manipulating the data. <code>Args</code> objects do not process the depsets they encapsulate until the execution phase, when it comes time to calculate the command line. This helps defer any expensive copying until after the analysis phase is complete. See the <a href='https://bazel.build/rules/performance'>Optimizing Performance</a> page for more information.<p><code>Args</code> are constructed by calling <a href='../builtins/actions.html#args'><code>ctx.actions.args()</code></a>. They can be passed as the <code>arguments</code> parameter of <a href='../builtins/actions.html#run'><code>ctx.actions.run()</code></a> or <a href='../builtins/actions.html#run_shell'><code>ctx.actions.run_shell()</code></a>. Each mutation of an <code>Args</code> object appends values to the eventual command line.<p>The <code>map_each</code> feature allows you to customize how items are transformed into strings. If you do not provide a <code>map_each</code> function, the standard conversion is as follows: <ul><li>Values that are already strings are left as-is.<li><a href='../builtins/File.html'><code>File</code></a> objects are turned into their     <code>File.path</code> values.<li>All other types are turned into strings in an <i>unspecified</i> manner. For     this reason, you should avoid passing values that are not of string or     <code>File</code> type to <code>add()</code>, and if you pass them to     <code>add_all()</code> or <code>add_joined()</code> then you should provide a     <code>map_each</code> function.</ul><p>When using string formatting (<code>format</code>, <code>format_each</code>, and <code>format_joined</code> params of the <code>add*()</code> methods), the format template is interpreted in the same way as <code>%</code>-substitution on strings, except that the template must have exactly one substitution placeholder and it must be <code>%s</code>. Literal percents may be escaped as <code>%%</code>. Formatting is applied after the value is converted to a string as per the above.<p>Each of the <code>add*()</code> methods have an alternate form that accepts an extra positional parameter, an "arg name" string to insert before the rest of the arguments. For <code>add_all</code> and <code>add_joined</code> the extra string will not be added if the sequence turns out to be empty. For instance, the same usage can add either <code>--foo val1 val2 val3 --bar</code> or just <code>--bar</code> to the command line, depending on whether the given sequence contains <code>val1..val3</code> or is empty.<p>If the size of the command line can grow longer than the maximum size allowed by the system, the arguments can be spilled over into parameter files. See <a href='#use_param_file'><code>use_param_file()</code></a> and <a href='#set_param_file_format'><code>set_param_file_format()</code></a>.<p>Example: Suppose we wanted to generate the command line: <pre>
--foo foo1.txt foo2.txt ... fooN.txt --bar bar1.txt,bar2.txt,...,barM.txt --baz
</pre>We could use the following <code>Args</code> object: <pre class=language-python>
# foo_deps and bar_deps are depsets containing
# File objects for the foo and bar .txt files.
args = ctx.actions.args()
args.add_all("--foo", foo_deps)
args.add_joined("--bar", bar_deps, join_with=",")
args.add("--baz")
ctx.actions.run(
  ...
  arguments = [args],
  ...
)
</pre>
ﬂ
Aspect‘For more information about Aspects, please consult the <a href="../globals/bzl.html#aspect">documentation of the aspect function</a> or the <a href="https://bazel.build/rules/aspects">introduction to Aspects</a>.
ç
	AttributeˇRepresentation of a definition of an attribute. Use the <a href="../toplevel/attr.html">attr</a> module to create an Attribute. They are only for use with a <a href="../globals/bzl.html#rule">rule</a> or an <a href="../globals/bzl.html#aspect">aspect</a>.
À
bazel_module8
is_rootbool"'Whether this module is the root module.'
namestring"The name of the module.l
tagsbazel_module_tags"QThe tags in the module related to the module extension currently being processed.-
versionstring"The version of the module.;Represents a Bazel module in the external dependency graph.
Ã
bazel_module_tags∂Contains the tags in a module for the module extension currently being processed. This object has a field for each tag class of the extension, and the value of the field is a list containing an object for each tag instance. This "tag instance" object in turn has a field for each attribute of the tag class.

When passed as positional arguments to <code>print()</code> or <code>fail()</code>, tag instance objects turn into a meaningful string representation of the form "'install' tag at /home/user/workspace/MODULE.bazel:3:4". This can be used to construct error messages that point to the location of the tag in the module file, e.g. <code>fail("Conflict between", tag1, "and", tag2)</code>.
—
BuildSetting¿The descriptor for a single piece of configuration information. If configuration is a key-value map of settings like {'cpu': 'ppc', 'copt': '-DFoo'}, this describes a single entry in that map.
ü
CcCompilationOutputs*
objectssequence"Non-PIC object files.*
pic_objectssequence"PIC object files./Helper class containing CC compilation outputs.
Ë
CcLinkingOutputs5

executableFile"!Represents the linked executable.l
library_to_linkLibraryToLink"J<code>LibraryToLink</code> for including these outputs in further linking./Helper class containing CC compilation outputs.
ë
CompilationContextÿ
definesdepset"ƒReturns the set of defines needed to compile this target. Each define is a string. These values are propagated to the target's transitive dependents, that is, any rules that depend on this target.◊
direct_headerslist"æReturns the list of modular headers that are declared by this target. This includes both public headers (such as those listed in "hdrs") and private headers (such as those listed in "srcs").Ü
direct_private_headerslist"fReturns the list of modular private headers (those listed in "srcs") that are declared by this target.Ñ
direct_public_headerslist"eReturns the list of modular public headers (those listed in "hdrs") that are declared by this target.e
direct_textual_headerslist"EReturns the list of textual headers that are declared by this target.ü
external_includesdepset"ÅReturns the set of search paths (as strings) for external header files referenced by angle bracket. Usually passed with -isystem.~
framework_includesdepset"`Returns the set of search paths (as strings) for framework header files. Usually passed with -F.L
headersdepset"9Returns the set of headers needed to compile this target.ó
includesdepset"ÇReturns the set of search paths (as strings) for header files referenced both by angle bracket and quotes. Usually passed with -I.≥
local_definesdepset"ôReturns the set of defines needed to compile this target. Each define is a string. These values are not propagated to the target's transitive dependents.Á
quote_includesdepset"ÃReturns the set of search paths (as strings) for header files referenced by quotes, e.g. #include "foo/bar/header.h". They can be either relative to the exec root or absolute. Usually passed with -iquote.˜
system_includesdepset"€Returns the set of search paths (as strings) for header files referenced by angle brackets, e.g. #include &lt;foo/bar/header.h&gt;. They can be either relative to the exec root or absolute. Usually passed with -isystem.H
validation_artifactsdepset"(Returns the set of validation artifacts.aImmutable store of information needed for C++ compilation that is aggregated across dependencies.
Í
configurationÃ
coverage_enabledbool"±A boolean that tells whether code coverage is enabled for this run. Note that this does not compute whether a specific rule should be instrumented for code coverage data collection. For that, see the <a href="../builtins/ctx.html#coverage_instrumented"><code>ctx.coverage_instrumented</code></a> function.Ö
default_shell_envdict"jA dictionary representing the static local shell environment. It maps variables to their values (strings).i
host_path_separatorstring"JReturns the separator for PATH environment variable, which is ':' on Unix.∫
test_envdict"ßA dictionary containing user-specified test environment variables and their values, as set by the --test_env options. DO NOT USE! This is not the complete environment!ŸThis object holds information about the environment in which the build is running. See the <a href='https://bazel.build/extending/rules#configurations'>Rules page</a> for more on the general concept of configurations.
≤Ç
ctxb
actionsactions"NContains methods for declaring output files and the actions that produce them.{

aspect_idslist"gA list of ids for all aspects applied to the target. Only available in aspect implementation functions.±
attrstruct"†A struct to access the values of the <a href='https://bazel.build/extending/rules#attributes'>attributes</a>. The values are provided by the user (if not, a default value is used). The attributes of the struct and the types of their values correspond to the keys and values of the <a href='../globals/bzl.html#rule.attrs'><code>attrs</code> dict</a> provided to the <a href='../globals/bzl.html#rule'><code>rule</code> function</a>. <a href="https://github.com/bazelbuild/examples/blob/main/rules/attributes/printer.bzl">See example of use</a>.9
bin_dirroot"(The root corresponding to bin directory.ù
build_file_pathstring"ÅDeprecated: Use <code>ctx.label.package + '/BUILD'</code>. The path to the BUILD file for this rule, relative to the source root.¢
build_setting_valueunknown"ÅValue of the build setting represented by the current target. If this isn't the context for an instance of a rule that sets the <a href="https://bazel.build/extending/config#rule-parameter"><code>build_setting</code></a> attribute, reading this is an error.î
configurationconfiguration"tThe default configuration. See the <a href="../builtins/configuration.html">configuration</a> type for more details.ü
coverage_instrumented∂
≠
targetQ<a class="anchor" href="../builtins/Target.html">Target</a>; or <code>None</code>JA Target specifying a rule. If not provided, defaults to the current rule."Nonebool"ÃReturns whether code coverage instrumentation should be generated when performing compilation actions for this rule or, if <code>target</code> is provided, the rule specified by that Target. (If a non-rule or a Starlark rule Target is provided, this returns False.) Checks if the sources of the current rule (if no Target is provided) or the sources of Target should be instrumented based on the --instrumentation_filter and --instrument_test_targets config settings. This differs from <code>coverage_enabled</code> in the <a href="../builtins/configuration.html">configuration</a>, which notes whether coverage data collection is enabled for the entire run, but not whether a specific target should be instrumented.Ë
created_actionsStarlarkValue"√For rules with <a href="../globals/bzl.html#rule._skylark_testable">_skylark_testable</a> set to <code>True</code>, this returns an <code>Actions</code> provider representing all actions created so far for the current rule. For all other rules, returns <code>None</code>. Note that the provider is not updated when subsequent actions are created, so you will have to call this function again if you wish to inspect them. <br/><br/>This is intended to help write tests for rule-implementation helper functions, which may take in a <code>ctx</code> object and create actions on it.f
disabled_featureslist"KThe set of features that are explicitly disabled by the user for this rule.≤
exec_groupsExecGroupCollection"çA collection of the execution groups available for this rule, indexed by their name. Access with <code>ctx.exec_groups[name_of_group]</code>.‚

executablestruct"ÀA <code>struct</code> containing executable files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.executable'><code>executable=True</code></a>. The struct fields correspond to the attribute names. Each value in the struct is either a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>executable=True</code>, no corresponding struct field is generated. <a href="https://github.com/bazelbuild/examples/blob/main/rules/actions_run/execute.bzl">See example of use</a>.Å
expand_locationË
!
inputString to be expanded.(
∫
targetsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Target.html">Target</a>s2List of targets for additional lookup information."[]string"ÇExpands all <code>$(location ...)</code> templates in the given string by replacing <code>$(location //x)</code> with the path of the output file of target //x. Expansion only works for labels that point to direct dependencies of this rule or that are explicitly listed in the optional argument <code>targets</code>. <br/><br/><code>$(location ...)</code> will cause an error if the referenced target has multiple outputs. In this case, please use <code>$(locations ...)</code> since it produces a space-separated list of output paths. It can be safely used for a single output file, too.<br/><br/>This function is useful to let the user specify a command in a BUILD file (like for <code>genrule</code>). In other cases, it is often better to manipulate labels directly.¸
expand_make_variablesÖ
A
attribute_name-The attribute name. Used for error reporting.(
U
commandHThe expression to expand. It can contain references to "Make variables".(
a
additional_substitutionsCAdditional substitutions to make beyond the default make variables.(string"⁄<b>Deprecated.</b> Use <a href="../builtins/ctx.html#var">ctx.var</a> to access the variables instead.<br>Returns a string after expanding all references to "Make variables". The variables must have the following format: <code>$(VAR_NAME)</code>. Also, <code>$$VAR_NAME</code> expands to <code>$VAR_NAME</code>. Examples:<pre class=language-python>
ctx.expand_make_variables("cmd", "$(MY_VAR)", {"MY_VAR": "Hi"})  # == "Hi"
ctx.expand_make_variables("cmd", "$$PWD", {})  # == "$PWD"
</pre>Additional variables may come from other places, such as configurations. Note that this function is experimental.»
featureslist"µThe set of features that are explicitly enabled by the user for this rule. <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/rule.bzl">See example of use</a>.Ÿ
filestruct"»A <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.allow_single_file'><code>allow_single_file</code></a>. The struct fields correspond to the attribute names. The struct value is always a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>allow_single_file</code>, no corresponding struct field is generated. It is a shortcut for:<pre class=language-python>list(ctx.attr.&lt;ATTR&gt;.files)[0]</pre>In other words, use <code>file</code> to access the (singular) <a href="https://bazel.build/extending/rules#requesting_output_files">default output</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/expand_template/hello.bzl">See example of use</a>.Œ
filesstruct"ºA <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label</a> or <a href='../toplevel/attr.html#label_list'>label list</a> type attributes. The struct fields correspond to the attribute names. The struct values are <code>list</code> of <a href='../builtins/File.html'><code>File</code></a>s.  It is a shortcut for:<pre class=language-python>[f for t in ctx.attr.&lt;ATTR&gt; for f in t.files]</pre> In other words, use <code>files</code> to access the <a href="https://bazel.build/extending/rules#requesting_output_files"> default outputs</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/depsets/foo.bzl">See example of use</a>.Y
	fragments	fragments"AAllows access to configuration fragments in target configuration.C
genfiles_dirroot"-The root corresponding to genfiles directory.π
	info_fileFile"•The file that is used to hold the non-volatile workspace status for the current build request. See documentation for --workspace_status_command for more information.A
labelLabel"1The label of the target currently being analyzed.Ö
outputs	structure"ÓA pseudo-struct containing all the predeclared output files, represented by <a href='../builtins/File.html'><code>File</code></a> objects. See the <a href='https://bazel.build/extending/rules#files'>Rules page</a> for more information and examples.<p>This field does not exist on aspect contexts, since aspects do not have predeclared outputs.<p>The fields of this object are defined as follows. It is an error if two outputs produce the same field name or have the same label.<ul><li> If the rule declares an <a href='../globals/bzl.html#rule.outputs'><code>outputs</code></a> dict, then for every entry in the dict, there is a field whose name is the key and whose value is the corresponding <code>File</code>.<li>For every attribute of type <a href='../toplevel/attr.html#output'><code>attr.output</code></a> that the rule declares, there is a field whose name is the attribute's name. If the target specified a label for that attribute, then the field value is the corresponding <code>File</code>; otherwise the field value is <code>None</code>.<li>For every attribute of type <a href='../toplevel/attr.html#output_list'><code>attr.output_list</code></a> that the rule declares, there is a field whose name is the attribute's name. The field value is a list of <code>File</code> objects corresponding to the labels given for that attribute in the target, or an empty list if the attribute was not specified in the target.<li><b>(Deprecated)</b> If the rule is marked <a href='../globals/bzl.html#rule.executable'><code>executable</code></a> or <a href='../globals/bzl.html#rule.test'><code>test</code></a>, there is a field named <code>"executable"</code>, which is the default executable. It is recommended that instead of using this, you pass another file (either predeclared or not) to the <code>executable</code> arg of <a href='../providers/DefaultInfo.html'><code>DefaultInfo</code></a>.</ul>Ü
resolve_command¢
"
commandCommand to resolve."''
®
	attributeM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>FName of the associated attribute for which to issue an error, or None."None
å
expand_locationsqShall we expand $(location) variables? See <a href="#expand_location">ctx.expand_location()</a> for more details."False
Ö
make_variablesI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>"Make variables to expand, or None."None
¶
toolsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Target.html">Target</a>s List of tools (list of targets)."[]
v

label_dictdDictionary of resolved labels and the corresponding list of Files (a dict of Label : list of Files)."{}
ë
execution_requirementssInformation for scheduling the action to resolve this command. See <a href="#common.tags">tags</a> for useful keys."{}tuple"Õ<i>(Experimental)</i> Returns a tuple <code>(inputs, command, input_manifests)</code> of the list of resolved inputs, the argv list for the resolved command, and the runfiles metadata required to run the command, all of them suitable for passing as the same-named arguments of the <code>ctx.action</code> method.<br/><b>Note for Windows users</b>: this method requires Bash (MSYS2). Consider using <code>resolve_tools()</code> instead (if that fits your needs).Ê
resolve_tools∞
¶
toolsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Target.html">Target</a>s List of tools (list of targets)."[]tuple"°Returns a tuple <code>(inputs, input_manifests)</code> of the depset of resolved inputs and the runfiles metadata required to run the tools, both of them suitable for passing as the same-named arguments of the <code>ctx.actions.run</code> method.<br/><br/>In contrast to <code>ctx.resolve_command</code>, this method does not require that Bash be installed on the machine, so it's suitable for rules built on Windows.í
rulerule_attributes"yRule attributes descriptor for the rule that the aspect is applied to. Only available in aspect implementation functions.™
runfilesÅ
∞
filess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s.The list of files to be added to the runfiles."[]
»
transitive_filesç<a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <code>None</code>ùThe (transitive) set of files to be added to the runfiles. The depset should use the <code>default</code> order (which, as the name implies, is the default)."None
¯
collect_data‡<b>Use of this parameter is not recommended. See <a href="https://bazel.build/extending/rules#runfiles">runfiles guide</a></b>. <p>Whether to collect the data runfiles from the dependencies in srcs, data and deps attributes."False
˛
collect_default„<b>Use of this parameter is not recommended. See <a href="https://bazel.build/extending/rules#runfiles">runfiles guide</a></b>. <p>Whether to collect the default runfiles from the dependencies in srcs, data and deps attributes."False
„
symlinksø<a class="anchor" href="../core/dict.html">dict</a>; or <a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../builtins/SymlinkEntry.html">SymlinkEntry</a>sêEither a SymlinkEntry depset or the map of symlinks to be added to the runfiles. Symlinks are always added under the main workspace's runfiles directory (e.g. <code>&lt;runfiles_root>/_main/&lt;symlink_path></code>, <b>not</b> the directory corresponding to the current target's repository. See <a href="https://bazel.build/extending/rules#runfiles_symlinks">Runfiles symlinks</a> in the rules guide."{}
î
root_symlinksø<a class="anchor" href="../core/dict.html">dict</a>; or <a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../builtins/SymlinkEntry.html">SymlinkEntry</a>sºEither a SymlinkEntry depset or a map of symlinks to be added to the runfiles. See <a href="https://bazel.build/extending/rules#runfiles_symlinks">Runfiles symlinks</a> in the rules guide."{}runfiles"Creates a runfiles object.≠

split_attrstruct"ñA struct to access the values of attributes with split configurations. If the attribute is a label list, the value of split_attr is a dict of the keys of the split (as strings) to lists of the ConfiguredTargets in that branch of the split. If the attribute is a label, then the value of split_attr is a dict of the keys of the split (as strings) to single ConfiguredTargets. Attributes with split configurations still appear in the attr struct, but their values will be single lists with all the branches of the split merged together.b
super	unknown"NExperimental: Calls parent's implementation function and returns its providersÕ
target_platform_has_constraintW
O
constraintValue:The constraint value to check the target platform against.(bool"RReturns true if the given constraint value is part of the current target platform.S

toolchainsToolchainContext"3Toolchains for the default exec group of this rule.F
vardict"9Dictionary (String to String) of configuration variables.∏
version_fileFile"°The file that is used to hold the volatile workspace status for the current build request. See documentation for --workspace_status_command for more information.•
workspace_namestring"äThe name of the workspace, which is effectively the execution root name and runfiles prefix for the main repo. If <code>--enable_bzlmod</code> is on, this is the fixed string <code>_main</code>. Otherwise, this is the workspace name as defined in the WORKSPACE file.µA context object that is passed to the implementation function for a rule or aspect. It provides access to the information and methods needed to analyze the current target.<p>In particular, it lets the implementation function access the current target's label, attributes, configuration, and the providers of its dependencies. It has methods for declaring output files and the actions that produce them.<p>Context objects essentially live for the duration of the call to the implementation function. It is not useful to access these objects outside of their associated function. See the <a href='https://bazel.build/extending/rules#implementation_function'>Rules page</a> for more information.
´
depsetø
to_listlist"´Returns a list of the elements, without duplicates, in the depset's traversal order. Note that order is unspecified (but deterministic) for elements that were added more than once to the depset. Order is also unspecified for <code>"default"</code>-ordered depsets, and for elements of child depsets whose order differs from that of the parent depset. The list is a copy; modifying it has no effect on the depset and vice versa.ﬁ<p>A specialized data structure that supports efficient merge operations and has a defined traversal order. Commonly used for accumulating data from transitive dependencies in rules and aspects. For more information see <a href="/extending/depsets">here</a>. <p>The elements of a depset must be hashable and all of the same type (as defined by the built-in type(x) function), but depsets are not simply hash sets and do not support fast membership tests. If you need a general set datatype, you can simulate one using a dictionary where all keys map to <code>True</code>.<p>Depsets are immutable. They should be created using their <a href="../globals/bzl.html#depset">constructor function</a> and merged or augmented with other depsets via the <code>transitive</code> argument. <p>The <code>order</code> parameter determines the kind of traversal that is done to convert the depset to an iterable. There are four possible values:<ul><li><code>"default"</code> (formerly <code>"stable"</code>): Order is unspecified (but deterministic).</li><li><code>"postorder"</code> (formerly <code>"compile"</code>): A left-to-right post-ordering. Precisely, this recursively traverses all children leftmost-first, then the direct elements leftmost-first.</li><li><code>"preorder"</code> (formerly <code>"naive_link"</code>): A left-to-right pre-ordering. Precisely, this traverses the direct elements leftmost-first, then recursively traverses the children leftmost-first.</li><li><code>"topological"</code> (formerly <code>"link"</code>): A topological ordering from the root down to the leaves. There is no left-to-right guarantee.</li></ul><p>Two depsets may only be merged if either both depsets have the same order, or one of them has <code>"default"</code> order. In the latter case the resulting depset's order will be the same as the other's order.<p>Depsets may contain duplicate values but these will be suppressed when iterating (using <code>to_list()</code>). Duplicates may interfere with the ordering semantics.
ª
DirectoryExpanderê
expand2
*
file The directory or file to expand.(list"—If the given <code>File</code> is a directory, this returns a list of <code>File</code>s recursively underneath the directory. Otherwise, this returns a list containing just the given <code>File</code> itself.íExpands directories created by <a href='../builtins/actions.html#declare_directory'><code>ctx.actions.declare_directory</code></a> during the execution phase. This is useful to expand directories in <a href='../builtins/Args.html#add_all.map_each'><code>map_each</code></a>.
ó
DottedVersion§

compare_to+
$
otherThe other dotted version.(int"iCompares based on most significant (first) not-matching version component. So, for example, 1.2.3 < 1.2.4_A value representing a version with multiple components, separated by periods, such as 1.2.3.4.
•
exec_resultª
return_codeint"¶The return code returned after the execution of the program. 256 if the process was terminated by a time out; values larger than 128 indicate termination by a signal.U
stderrstring"CThe content of the standard error output returned by the execution.O
stdoutstring"=The content of the standard output returned by the execution.ØA structure storing result of repository_ctx.execute() method. It contains the standard output stream content, the standard error stream content and the execution return code.
D
ExecGroupCollection-Stores exec groups available to a given rule.
Ñ
ExecGroupContextG

toolchainsToolchainContext"'Toolchains required for this exec group'Stores information about an exec group.
1
ExecTransitionFactoryan execution transition.
≥
extension_metadataúReturn values of this type from a module extension's implementation function to provide metadata about the repositories generated by the extension to Bazel.
V
FeatureConfiguration>Class used to construct command lines from CROSSTOOL features.
Ü
Fileb
basenamestring"NThe base name of this file. This is the name of the file inside the directory.ü
dirnamestring"ãThe name of the directory containing this file. It's taken from <a href="#path">path</a> and is always relative to the execution directory.ü
	extensionstring"âThe file extension of this file, following (not including) the rightmost period. Empty string if the file's basename includes no periods.:
is_directorybool"$Returns true if this is a directory.S
	is_sourcebool"@Returns true if this is a source file, i.e. it is not generated.<
ownerLabel",A label of a target that produces this File.Ö
pathstring"ÙThe execution path of this file, relative to the workspace's execution directory. It consists of two parts, an optional first part called the <i>root</i> (see also the <a href="../builtins/root.html">root</a> module), and the second part which is the <code>short_path</code>. The root may be empty, which it usually is for non-generated files. For generated files it usually contains a configuration-specific path fragment that encodes things like the target CPU architecture that was used while building said file. Use the <code>short_path</code> for the path under which the file is mapped if it's in the runfiles of a binary.7
rootroot")The root beneath which this file resides.˝

short_pathstring"ÊThe path of this file relative to its root. This excludes the aforementioned <i>root</i>, i.e. configuration-specific fragments of the path. This is also the path under which the file is mapped if it's in the runfiles of a binary.ï
tree_relative_pathstring"ˆThe path of this file relative to the root of the ancestor's tree, if the ancestor's <a href="#is_directory">is_directory</a> field is true. <code>tree_relative_path</code> is only available for expanded files of a directory in an action command, i.e. <a href="../builtins/Args.html#add_all">Args.add_all()</a>. For other types of files, it is an error to access this field.≠This object is created during the analysis phase to represent a file or directory that will be read or written during the execution phase. It is not an open file handle, and cannot be used to directly read or write file contents. Rather, you use it to construct the action graph in a rule implementation function by passing it to action-creating functions. See the <a href='https://bazel.build/extending/rules#files'>Rules page</a> for more information.<p>When a <code>File</code> is passed to an <a href='../builtins/Args.html'><code>Args</code></a> object without using a <code>map_each</code> function, it is converted to a string by taking the value of its <code>path</code> field.
†
	fragmentsíA collection of configuration fragments available in the current rule implementation context. Access a specific fragment by its field name. For example, <code>ctx.fragments.java</code> <p>Only configuration fragments which are declared in the rule definition may be accessed in this collection.</p><p>See the <a href="../fragments.html">configuration fragment reference</a> for a list of available fragments and the <a href="https://bazel.build/extending/rules#configuration_fragments">rules documentation</a> for how to use them.
¥
java_annotation_processingi
	class_jarFile"VDeprecated: Please use <code>JavaInfo.java_outputs.generated_class_jar</code> instead.^
enabledbool"MDeprecated. Returns true if annotation processing was applied on this target.ü
processor_classnameslist"ÄDeprecated: Please use <code>JavaInfo.plugins</code> instead. Returns class names of annotation processors applied to this rule.†
processor_classpathdepset"ÄDeprecated: Please use <code>JavaInfo.plugins</code> instead. Returns a classpath of annotation processors applied to this rule.k

source_jarFile"WDeprecated: Please use <code>JavaInfo.java_outputs.generated_source_jar</code> instead.†
transitive_class_jarsdepset"Deprecated. Returns a transitive set of class file jars resulting from annotation processing of this rule and its dependencies.°
transitive_source_jarsdepset"Deprecated. Returns a transitive set of source archives resulting from annotation processing of this rule and its dependencies.RInformation about jars that are a result of annotation processing for a Java rule.
∆
Labelû
namestring"çThe name of the target referred to by this label. For instance:<br><pre class=language-python>Label("@@foo//pkg/foo:abc").name == "abc"</pre>›
packagestring"…The name of the package containing the target referred to by this label, without the repository name. For instance:<br><pre class=language-python>Label("@@repo//pkg/foo:abc").package == "pkg/foo"</pre>ù
relativeK
B
relName5The label that will be resolved relative to this one.(Label"√<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> This method behaves surprisingly when used with an argument containing an apparent repo name. Prefer <a href="#local_target_label"><code>Label.same_package_label()</code></a>, <a href="../toplevel/native#package_relative_label"><code>native.package_relative_label()</code></a>, or <a href="#Label"><code>Label()</code></a> instead.<p>Resolves a label that is either absolute (starts with <code>//</code>) or relative to the current package. If this label is in a remote repository, the argument will be resolved relative to that repository. If the argument contains a repository name, the current label is ignored and the argument is returned as-is, except that the repository name is rewritten if it is in the current repository mapping. Reserved labels will also be returned as-is.<br>For example:<br><pre class=language-python>
Label("//foo/bar:baz").relative(":quux") == Label("//foo/bar:quux")
Label("//foo/bar:baz").relative("//wiz:quux") == Label("//wiz:quux")
Label("@repo//foo/bar:baz").relative("//wiz:quux") == Label("@repo//wiz:quux")
Label("@repo//foo/bar:baz").relative("//visibility:public") == Label("//visibility:public")
Label("@repo//foo/bar:baz").relative("@other//wiz:quux") == Label("@other//wiz:quux")
</pre><p>If the repository mapping passed in is <code>{'@other' : '@remapped'}</code>, then the following remapping will take place:<br><pre class=language-python>
Label("@repo//foo/bar:baz").relative("@other//wiz:quux") == Label("@remapped//wiz:quux")
</pre>Ù
	repo_namestring"ﬁThe canonical name of the repository containing the target referred to by this label, without any leading at-signs (<code>@</code>). For instance, <pre class=language-python>Label("@@foo//bar:baz").repo_name == "foo"</pre>†
same_package_label;
2
target_name!The target name of the new label.(Label"MCreates a label in the same package as this label with the given target name.á
workspace_namestring"Ï<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> The field name "workspace name" is a misnomer here; use the identically-behaving <a href="#repo_name"><code>Label.repo_name</code></a> instead.<p>The canonical name of the repository containing the target referred to by this label, without any leading at-signs (<code>@</code>). For instance, <pre class=language-python>Label("@@foo//bar:baz").workspace_name == "foo"</pre>É
workspace_rootstring"ËReturns the execution root for the repository containing the target referred to by this label, relative to the execroot. For instance:<br><pre class=language-python>Label("@repo//pkg/foo:abc").workspace_root == "external/repo"</pre>ÒA BUILD target identifier.<p>For every <code>Label</code> instance <code>l</code>, the string representation <code>str(l)</code> has the property that <code>Label(str(l)) == l</code>, regardless of where the <code>Label()</code> call occurs.
¬
LateBoundDefault≠Represents a late-bound default attribute value of type 'Label'. The value of a LateBoundDefault is only resolvable in the context of a rule implementation function, and depends on the current build configuration. For example, a LateBoundDefault might represent the Label of the java toolchain in the current build configuration. <p>See <a href="../globals/bzl.html#configuration_field">configuration_field</a> for example usage.
á

LibraryToLink\

alwayslinkbool"HWhether to link the static library/objects in the --whole_archive block.Æ
dynamic_libraryFile"î<code>Artifact</code> of dynamic library to be linked. Always used for runtime and used for linking if <code>interface_library</code> is not passed.S
interface_libraryFile"8<code>Artifact</code> of interface library to be linked.U
lto_bitcode_filessequence"6<code>List</code> of LTO bitcode files in the library.F
objectssequence"1<code>List</code> of object files in the library.]
pic_lto_bitcode_filessequence":<code>List</code> of pic LTO bitcode files in the library.N
pic_objectssequence"5<code>List</code> of pic object files in the library.U
pic_static_libraryFile"9<code>Artifact</code> of pic static library to be linked.∑
 resolved_symlink_dynamic_libraryFile"åThe resolved <code>Artifact</code> of the dynamic library to be linked if <code>dynamic_library</code> is a symlink, otherwise this is None.Ω
"resolved_symlink_interface_libraryFile"êThe resolved <code>Artifact</code> of the interface library to be linked if <code>interface_library</code> is a symlink, otherwise this is None.M
static_libraryFile"5<code>Artifact</code> of static library to be linked.$A library the user can link against.
å
LicenseÄThis API is deprecated and will be removed. Please do not depend on it. This object represents the value of a license attribute.
Ã
LinkerInput]
additional_inputssequence">Returns the depset of additional inputs, e.g.: linker scripts.}
	librariessequence"fReturns the depset of <code>LibraryToLink</code>. May return a list but this is deprecated. See #8118.6
ownerLabel"&Returns the owner of this LinkerInput.S
user_link_flagssequence"6Returns the list of user link flags passed as strings.REither libraries, flags or other files that may be passed to the linker as inputs.
Æ
LinkingContext=
linker_inputsdepset"$Returns the depset of linker inputs.]Immutable store of information needed for C++ linking that is aggregated across dependencies.
õa

module_ctxò
downloadÍ
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
Ä
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>>path to the output file, relative to the repository directory."''
À
sha256ºthe expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping."''
S

executable>set the executable flag on the created file, false by default."False
t

allow_fail_If set, indicate the error in the return value instead of raising an error for failed downloads"False

canonical_idkIf set, restrict cache hits to those cases where the file was added to the cache with the same canonical id"''
X
authLAn optional dict specifying authentication information for some of the URLs."{}
E
headers6An optional dict specifying http headers for all URLs."{}
‰
	integrity“Expected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping."''
¸
blockÏIf set to false, the call returns immediately and instead of the regular return value, it returns a token with one single method, wait(), which blocks until the download is finished and returns the usual return value or throws as usual."Trueunknown"ûDownloads a file to the output path for the provided url and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>.‘
download_and_extractå
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
°
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>_path to the directory where the archive will be unpacked, relative to the repository directory."''
Ø
sha256†the expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given hash; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache."''
»
typeªthe archive type of the downloaded file. By default, the archive type is determined from the file extension of the URL. If the file has no extension, you can explicitly specify either "zip", "jar", "war", "aar", "tar", "tar.gz", "tgz", "tar.xz", "txz", ".tar.zst", ".tzst", "tar.bz2", ".tbz", ".ar", or ".deb" here."''
Æ
stripPrefixöa directory prefix to strip from the extracted files.
Many archives contain a top-level directory that contains all files in the archive. Instead of needing to specify this prefix over and over in the <code>build_file</code>, this field can be used to strip it from extracted files."''
t

allow_fail_If set, indicate the error in the return value instead of raising an error for failed downloads"False

canonical_idkIf set, restrict cache hits to those cases where the file was added to the cache with the same canonical id"''
X
authLAn optional dict specifying authentication information for some of the URLs."{}
E
headers6An optional dict specifying http headers for all URLs."{}
‰
	integrity“Expected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping."''
Û
rename_filesﬁAn optional dict specifying files to rename during the extraction. Archive entries with names exactly matching a key will be renamed to the value, prior to any directory prefix adjustment. This can be used to extract archives that contain non-Unicode filenames, or which have files that would extract to the same path on case-insensitive filesystems."{}struct"¨Downloads a file to the output path for the provided url, extracts it, and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>.›
execute·
a
	argumentsRList of arguments, the first element should be the path to the program to execute.(
T
timeoutDmaximum duration of the command in seconds (default is 600 seconds)."600
Z
environmentGforce some environment variables to be set to be passed to the process."{}
F
quiet7If stdout and stderr should be printed to the terminal."True
u
working_directory\Working directory for command execution.
Can be relative to the repository root or absolute."""exec_result"ÌExecutes the command given by the list of arguments. The execution time of the command is limited by <code>timeout</code> (in seconds, default 600 seconds). This method returns an <code>exec_result</code> structure containing the output of the command. The <code>environment</code> map can be used to override some environment variables to be passed to the process.ø
extension_metadataÒ
°
root_module_direct_deps≈<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>∑The names of the repositories that the extension considers to be direct dependencies of the root module. If the root module imports additional repositories or does not import all of these repositories via <a href="../globals/module.html#use_repo"><code>use_repo</code></a>, Bazel will print a warning and a fixup command when the extension is evaluated.<p>If one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> is specified, the other has to be as well. The lists specified by these two parameters must be disjoint.<p>Exactly one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> can be set to the special value <code>"all"</code>, which is treated as if a list with the names of all repositories generated by the extension was specified as the value."None
∂	
root_module_direct_dev_deps≈<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>»The names of the repositories that the extension considers to be direct dev dependencies of the root module. If the root module imports additional repositories or does not import all of these repositories via <a href="../globals/module.html#use_repo"><code>use_repo</code></a> on an extension proxy created with <code><a href="../globals/module.html#use_extension">use_extension</a>(..., dev_dependency = True)</code>, Bazel will print a warning and a fixup command when the extension is evaluated.<p>If one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> is specified, the other has to be as well. The lists specified by these two parameters must be disjoint.<p>Exactly one of <code>root_module_direct_deps</code> and <code>root_module_direct_dev_deps</code> can be set to the special value <code>"all"</code>, which is treated as if a list with the names of all repositories generated by the extension was specified as the value."Noneextension_metadata"¥Constructs an opaque object that can be returned from the module extension's implementation function to provide metadata about the repositories generated by the extension to Bazel.ˇ
file≠
ˇ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Apath of the file to create, relative to the repository directory.(
C
content4the content of the file to create, empty by default."''
Q

executable=set the executable flag on the created file, true by default."True
Ü
legacy_utf8qencode file content to UTF-8, true by default. Future versions will change the default and remove this parameter."TrueNoneType"GGenerates a file in the repository directory with the provided content.õ
getenvÇ
g
name7<a class="anchor" href="../core/string.html">string</a>$name of desired environment variable(
é
defaultM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>.Default value to return if `name` is not found"Nonestring"ãReturns the value of an environment variable <code>name</code> as a string if exists, or <code>default</code> if it doesn't.<p>When building incrementally, any change to the value of the variable named by <code>name</code> will cause this repository to be re-fetched.æ
is_dev_dependencyz
r
tagbazel_module_tagWA tag obtained from <a href="../builtins/bazel_module.html#tags">bazel_module.tags</a>.(bool"¨Returns whether the given tag was specified on the result of a <a href="../globals/module.html#use_extension">use_extension</a> call with <code>devDependency = True</code>.˚
moduleslist"ÈA list of all the Bazel modules in the external dependency graph that use this module extension, each of which is a <a href="../builtins/bazel_module.html">bazel_module</a> object that exposes all the tags it specified for this extension. The iteration order of this dictionary is guaranteed to be the same as breadth-first search starting from the root module.D
osrepository_os"/A struct to access information from the system.®
path˝
Ù
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>6string, label or path from which to create a path from(path"üReturns a path from a string, label or path. If the path is relative, it will resolve relative to the repository directory. If the path is a label, it will resolve to the path of the corresponding file. Note that remote repositories are executed during the analysis phase and thus cannot depends on a target result (the label should point to a non-generated file). If path is a path, it will return that path as is.†
readÁ
‹
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>path of the file to read from.(string".Reads the content of a file on the filesystem.˜
report_progressé
Å
status7<a class="anchor" href="../core/string.html">string</a>:string describing the current status of the fetch progress"''NoneType"SUpdates the progress status for the fetching of this repository or module extensionp
"root_module_has_non_dev_dependencybool"DWhether the root module uses this extension as a non-dev dependency.ö
which1
)
programProgram to find in the path.(path"^Returns the path of the corresponding program or None if there is no such program in the path.¸The context of the module extension containing helper functions and information about pertinent tags across the dependency graph. You get a module_ctx object as an argument to the <code>implementation</code> function when you create a module extension.
≠
native_rule_transitioníRepresents a native transition that can be applied to a Starlark rule as an incoming edge. This is a valid value for cfg in rule() but not attr().
◊
path=
basenamestring")A string giving the basename of the file.`
dirnamepath"OThe parent directory of this file, or None if this file does not have a parent.E
existsbool"5Returns true if the file denoted by this path exists.Ÿ
	get_child}
u
*relative_paths^Zero or more relative path strings to append to this path with path separatorsadded as needed.(0path"MReturns the path obtained by joining this path with the given relative paths.M
readdirlist":The list of entries in the directory denoted by this path.{
realpathpath"iReturns the canonical path for this path by repeatedly replacing all symbolic links with their referents.?A structure representing a file to be used inside a repository.
‰
Provider◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.
Èh
repository_ctxá
attrstruct"wA struct to access the values of the attributes. The values are provided by the user (if not, a default value is used).¯
deleteÒ
Ë
paths<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/path.html">path</a>iPath of the file to delete, relative to the repository directory, or absolute. Can be a path or a string.(bool"zDeletes a file or a directory. Returns a bool, indicating whether the file or directory was actually deleted by this call.ò
downloadÍ
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
Ä
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>>path to the output file, relative to the repository directory."''
À
sha256ºthe expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping."''
S

executable>set the executable flag on the created file, false by default."False
t

allow_fail_If set, indicate the error in the return value instead of raising an error for failed downloads"False

canonical_idkIf set, restrict cache hits to those cases where the file was added to the cache with the same canonical id"''
X
authLAn optional dict specifying authentication information for some of the URLs."{}
E
headers6An optional dict specifying http headers for all URLs."{}
‰
	integrity“Expected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping."''
¸
blockÏIf set to false, the call returns immediately and instead of the regular return value, it returns a token with one single method, wait(), which blocks until the download is finished and returns the usual return value or throws as usual."Trueunknown"ûDownloads a file to the output path for the provided url and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>.‘
download_and_extractå
∫
urlÄ<a class="anchor" href="../core/string.html">string</a>; or Iterable of <a class="anchor" href="../core/string.html">string</a>s.List of mirror URLs referencing the same file.(
°
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>_path to the directory where the archive will be unpacked, relative to the repository directory."''
Ø
sha256†the expected SHA-256 hash of the file downloaded. This must match the SHA-256 hash of the file downloaded. It is a security risk to omit the SHA-256 as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping. If provided, the repository cache will first be checked for a file with the given hash; a download will only be attempted if the file was not found in the cache. After a successful download, the file will be added to the cache."''
»
typeªthe archive type of the downloaded file. By default, the archive type is determined from the file extension of the URL. If the file has no extension, you can explicitly specify either "zip", "jar", "war", "aar", "tar", "tar.gz", "tgz", "tar.xz", "txz", ".tar.zst", ".tzst", "tar.bz2", ".tbz", ".ar", or ".deb" here."''
Æ
stripPrefixöa directory prefix to strip from the extracted files.
Many archives contain a top-level directory that contains all files in the archive. Instead of needing to specify this prefix over and over in the <code>build_file</code>, this field can be used to strip it from extracted files."''
t

allow_fail_If set, indicate the error in the return value instead of raising an error for failed downloads"False

canonical_idkIf set, restrict cache hits to those cases where the file was added to the cache with the same canonical id"''
X
authLAn optional dict specifying authentication information for some of the URLs."{}
E
headers6An optional dict specifying http headers for all URLs."{}
‰
	integrity“Expected checksum of the file downloaded, in Subresource Integrity format. This must match the checksum of the file downloaded. It is a security risk to omit the checksum as remote files can change. At best omitting this field will make your build non-hermetic. It is optional to make development easier but should be set before shipping."''
Û
rename_filesﬁAn optional dict specifying files to rename during the extraction. Archive entries with names exactly matching a key will be renamed to the value, prior to any directory prefix adjustment. This can be used to extract archives that contain non-Unicode filenames, or which have files that would extract to the same path on case-insensitive filesystems."{}struct"¨Downloads a file to the output path for the provided url, extracts it, and returns a struct containing <code>success</code>, a flag which is <code>true</code> if the download completed successfully, and if successful, a hash of the file with the fields <code>sha256</code> and <code>integrity</code>.›
execute·
a
	argumentsRList of arguments, the first element should be the path to the program to execute.(
T
timeoutDmaximum duration of the command in seconds (default is 600 seconds)."600
Z
environmentGforce some environment variables to be set to be passed to the process."{}
F
quiet7If stdout and stderr should be printed to the terminal."True
u
working_directory\Working directory for command execution.
Can be relative to the repository root or absolute."""exec_result"ÌExecutes the command given by the list of arguments. The execution time of the command is limited by <code>timeout</code> (in seconds, default 600 seconds). This method returns an <code>exec_result</code> structure containing the output of the command. The <code>environment</code> map can be used to override some environment variables to be passed to the process.¶

extractÈ	
ë
archive±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Ppath to the archive that will be unpacked, relative to the repository directory.(
°
output±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>_path to the directory where the archive will be unpacked, relative to the repository directory."''
Æ
stripPrefixöa directory prefix to strip from the extracted files.
Many archives contain a top-level directory that contains all files in the archive. Instead of needing to specify this prefix over and over in the <code>build_file</code>, this field can be used to strip it from extracted files."''
Û
rename_filesﬁAn optional dict specifying files to rename during the extraction. Archive entries with names exactly matching a key will be renamed to the value, prior to any directory prefix adjustment. This can be used to extract archives that contain non-Unicode filenames, or which have files that would extract to the same path on case-insensitive filesystems."{}NoneType"/Extract an archive to the repository directory.ˇ
file≠
ˇ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Apath of the file to create, relative to the repository directory.(
C
content4the content of the file to create, empty by default."''
Q

executable=set the executable flag on the created file, true by default."True
Ü
legacy_utf8qencode file content to UTF-8, true by default. Future versions will change the default and remove this parameter."TrueNoneType"GGenerates a file in the repository directory with the provided content.õ
getenvÇ
g
name7<a class="anchor" href="../core/string.html">string</a>$name of desired environment variable(
é
defaultM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>.Default value to return if `name` is not found"Nonestring"ãReturns the value of an environment variable <code>name</code> as a string if exists, or <code>default</code> if it doesn't.<p>When building incrementally, any change to the value of the variable named by <code>name</code> will cause this repository to be re-fetched.I
namestring"9The name of the external repository created by this rule.D
osrepository_os"/A struct to access information from the system.Ô
patch∞
‘

patch_file±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>èThe patch file to apply, it can be label, relative path or absolute path. If it's a relative path, it will resolve to the repository directory.(
M
stripAstrip the specified number of leading components from file names."0NoneType"≤Apply a patch file to the root directory of external repository. The patch file should be a standard <a href="https://en.wikipedia.org/wiki/Diff#Unified_format">unified diff format</a> file. The Bazel-native patch implementation doesn't support fuzz match and binary patch like the patch command line tool.®
path˝
Ù
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>6string, label or path from which to create a path from(path"üReturns a path from a string, label or path. If the path is relative, it will resolve relative to the repository directory. If the path is a label, it will resolve to the path of the corresponding file. Note that remote repositories are executed during the analysis phase and thus cannot depends on a target result (the label should point to a non-generated file). If path is a path, it will return that path as is.†
readÁ
‹
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>path of the file to read from.(string".Reads the content of a file on the filesystem.˜
report_progressé
Å
status7<a class="anchor" href="../core/string.html">string</a>:string describing the current status of the fetch progress"''NoneType"SUpdates the progress status for the fetching of this repository or module extension∑
symlinkÖ
Í
target±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>*The path that the symlink should point to.(
ã
	link_name±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>HThe path of the symlink to create, relative to the repository directory.(NoneType"$Creates a symlink on the filesystem.‚
templateá
ˇ
path±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>Apath of the file to create, relative to the repository directory.(
‹
template±<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../builtins/path.html">path</a>path to the template file.(
G
substitutions2substitutions to make when expanding the template."{}
Q

executable=set the executable flag on the created file, true by default."TrueNoneType"ÀGenerates a new file using a <code>template</code>. Every occurrence in <code>template</code> of a key of <code>substitutions</code> will be replaced by the corresponding value. The result is written in <code>path</code>. An optional<code>executable</code> argument (default to true) can be set to turn on or offthe executable bit.ö
which1
)
programProgram to find in the path.(path"^Returns the path of the corresponding program or None if there is no such program in the path.O
workspace_rootpath"7The path to the root workspace of the bazel invocation.ﬁThe context of the repository rule containing helper functions and information about attributes. You get a repository_ctx object as an argument to the <code>implementation</code> function when you create a repository rule.
˚
repository_osç
archstring"}A string identifying the architecture Bazel is running on (the value of the "os.arch" Java property converted to lower case).Ü
environdict"ÙThe dictionary of environment variables.<p><b>NOTE</b>: Retrieving an environment variable from this dictionary does not establish a dependency from a repository rule or module extension to the environment variable.  To establish a dependency when looking up an environment variable, use either <code>repository_ctx.getenv</code> or <code>module_ctx.getenv</code> instead.í
namestring"ÅA string identifying the operating system Bazel is running on (the value of the "os.name" Java property converted to lower case).<Various data about the current platform Bazel is running on.
…
repository_ruleµA callable value that may be invoked during evaluation of the WORKSPACE file or within the implementation function of a module extension to instantiate and return a repository rule.
˘
rootP
pathstring"@Returns the relative path from the exec root to the actual root.ûA root for files. The roots are the directories containing files, and they are mapped together into a single directory tree to form the execution environment.
π
rule∞A callable value representing the type of a native or Starlark rule. Calling the value during evaluation of a package's BUILD file creates an instance of the rule and adds it to the package's target set. For more information, visit this page about<a href ='https://bazel.build/extending/rules'>Rules</a>.
“
rule_attributes±
attrstruct"†A struct to access the values of the <a href='https://bazel.build/extending/rules#attributes'>attributes</a>. The values are provided by the user (if not, a default value is used). The attributes of the struct and the types of their values correspond to the keys and values of the <a href='../globals/bzl.html#rule.attrs'><code>attrs</code> dict</a> provided to the <a href='../globals/bzl.html#rule'><code>rule</code> function</a>. <a href="https://github.com/bazelbuild/examples/blob/main/rules/attributes/printer.bzl">See example of use</a>.‚

executablestruct"ÀA <code>struct</code> containing executable files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.executable'><code>executable=True</code></a>. The struct fields correspond to the attribute names. Each value in the struct is either a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>executable=True</code>, no corresponding struct field is generated. <a href="https://github.com/bazelbuild/examples/blob/main/rules/actions_run/execute.bzl">See example of use</a>.Ÿ
filestruct"»A <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label type attributes</a> marked as <a href='../toplevel/attr.html#label.allow_single_file'><code>allow_single_file</code></a>. The struct fields correspond to the attribute names. The struct value is always a <a href='../builtins/File.html'><code>File</code></a> or <code>None</code>. If an optional attribute is not specified in the rule then the corresponding struct value is <code>None</code>. If a label type is not marked as <code>allow_single_file</code>, no corresponding struct field is generated. It is a shortcut for:<pre class=language-python>list(ctx.attr.&lt;ATTR&gt;.files)[0]</pre>In other words, use <code>file</code> to access the (singular) <a href="https://bazel.build/extending/rules#requesting_output_files">default output</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/expand_template/hello.bzl">See example of use</a>.Œ
filesstruct"ºA <code>struct</code> containing files defined in <a href='../toplevel/attr.html#label'>label</a> or <a href='../toplevel/attr.html#label_list'>label list</a> type attributes. The struct fields correspond to the attribute names. The struct values are <code>list</code> of <a href='../builtins/File.html'><code>File</code></a>s.  It is a shortcut for:<pre class=language-python>[f for t in ctx.attr.&lt;ATTR&gt; for f in t.files]</pre> In other words, use <code>files</code> to access the <a href="https://bazel.build/extending/rules#requesting_output_files"> default outputs</a> of a dependency. <a href="https://github.com/bazelbuild/examples/blob/main/rules/depsets/foo.bzl">See example of use</a>.8
kindstring"(The kind of a rule, such as 'cc_library'?Information about attributes of a rule an aspect is applied to.
Œ

runfilesB
empty_filenamesdepset"'Returns names of empty files to create.6
filesdepset"%Returns the set of runfiles as files.ô
merge>
2
other'The runfiles object to merge into this.(runfiles"œReturns a new runfiles object that includes all the contents of this one and the argument.<p><i>Note:</i> When you have many runfiles objects to merge, use <a href='#merge_all'><code>merge_all()</code></a> rather than calling <code>merge</code> in a loop. This avoids constructing deep depset structures which can cause build failures.Œ
	merge_all…
º
other{<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/runfiles.html">runfiles</a>s4The sequence of runfiles objects to merge into this.(runfiles"uReturns a new runfiles object that includes all the contents of this one and of the runfiles objects in the argument.:
root_symlinksdepset"!Returns the set of root symlinks.0
symlinksdepset"Returns the set of symlinks.ÍA container of information regarding a set of files required at runtime execution. This object should be passed via <a href="../providers/DefaultInfo.html">DefaultInfo</a> in order to tell the build system about the runfiles needed by the outputs produced by the rule. <p>See <a href="https://bazel.build/extending/rules#runfiles">runfiles guide</a> for details.
¿
structÂ	
to_jsonstring"œ	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a JSON string from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs, a list of these types or a dictionary with string keys and values of these types. Quotes and new lines in strings are escaped. Examples:<br><pre class=language-python>struct(key=123).to_json()
# {"key":123}

struct(key=True).to_json()
# {"key":true}

struct(key=[1, 2, 3]).to_json()
# {"key":[1,2,3]}

struct(key='text').to_json()
# {"key":"text"}

struct(key=struct(inner_key='text')).to_json()
# {"key":{"inner_key":"text"}}

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_json()
# {"key":[{"inner_key":1},{"inner_key":2}]}

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_json()
# {"key":{"inner_key":{"inner_inner_key":"text"}}}
</pre>.<p>Deprecated: instead, use json.encode(x) or json.encode_indent(x), which work for values other than structs and do not pollute the struct field namespace. ã

to_protostring"Ù	<b>Deprecated</b>. This API is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_struct_has_no_methods</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Creates a text message from the struct parameter. This method only works if all struct elements (recursively) are strings, ints, booleans, other structs or dicts or lists of these types. Quotes and new lines in strings are escaped. Struct keys are iterated in the sorted order. Examples:<br><pre class=language-python>struct(key=123).to_proto()
# key: 123

struct(key=True).to_proto()
# key: true

struct(key=[1, 2, 3]).to_proto()
# key: 1
# key: 2
# key: 3

struct(key='text').to_proto()
# key: "text"

struct(key=struct(inner_key='text')).to_proto()
# key {
#   inner_key: "text"
# }

struct(key=[struct(inner_key=1), struct(inner_key=2)]).to_proto()
# key {
#   inner_key: 1
# }
# key {
#   inner_key: 2
# }

struct(key=struct(inner_key=struct(inner_inner_key='text'))).to_proto()
# key {
#    inner_key {
#     inner_inner_key: "text"
#   }
# }

struct(foo={4: 3, 2: 1}).to_proto()
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre><p>Deprecated: use proto.encode_text(x) instead.øA generic object with fields.<p>Structs fields cannot be reassigned once the struct is created. Two structs are equal if they have the same fields and if corresponding field values are equal.
’
Subrule…Experimental: a building block for writing rules with shared code. For more information, please see the subrule proposal: https://docs.google.com/document/d/1RbNC88QieKvBEwir7iV5zZU08AaMlOzxhVkPnmKDedQ
¬
subrule_ctxa
actionsactions"MContains methods for declaring output files and the actions that produce themY
	fragments	fragments"AAllows access to configuration fragments in target configuration.@
labelLabel"0The label of the target currently being analyzedm

toolchainsToolchainContext"MContains methods for declaring output files and the actions that produce themDA context object passed to the implementation function of a subrule.
ø
SymlinkEntry<
pathstring",The path of the symlink in the runfiles tree/
target_fileFile"Target file of the symlink@A single runfiles symlink represented by a link name and target.
6
	tag_class)Defines a schema of attributes for a tag.
®
TargetùThe BUILD target for a dependency. Appears in the fields of <code><a href='../builtins/ctx.html#attr'>ctx.attr</a></code> corresponding to <a href='https://bazel.build/extending/rules#dependency_attributes'>dependency attributes</a> (<code><a href='../toplevel/attr.html#label'>label</a></code> or <code><a href='../toplevel/attr.html#label_list'>label_list</a></code>). Has the following fields:
<ul>
<li><h3 id='modules.Target.label'>label</h3>
<code><a href='../builtins/Label.html'>Label</a> Target.label</code><br/>
The identifier of the target.</li>
<li><h3 id='modules.Target.files'>files</h3>
<code><a href='../builtins/depset.html'>depset</a> Target.files </code><br/>
The set of <code><a href='../builtins/File.html'>File</a></code>s in the default outputs for this target. Equivalent to <code><a href='../providers/DefaultInfo.html#files'>target[DefaultInfo].files</a></code>.</li>
<li><h3 id='modules.Target.providers'>Providers</h3>
The <a href='https://bazel.build/extending/rules#providers'>providers</a> of a rule target can be accessed by type using index notation (<code>target[DefaultInfo]</code>). The presence of providers can be checked using the <code>in</code> operator (<code>SomeInfo in target</code>).<br/>
<br/>
If the rule's implementation function returns a <code><a href='../builtins/struct.html'>struct</a></code> instead of a list of <code><a href='../builtins/Provider.html'>Provider</a></code> instances, the struct's fields can be accessed via the corresponding fields of the <code>Target</code> (<code>target.some_legacy_info</code>). This behavior <a href='https://bazel.build/extending/rules#migrating_from_legacy_providers'>is deprecated</a>.</li>
</ul>
˛
TemplateDict[
add@

keyA String key(

valueA String value(TemplateDict"Add a String valueÉ

add_joinedﬁ


keyA String key(
o
values;<a class="anchor" href="../builtins/depset.html">depset</a>&The depset whose items will be joined.(
À
	join_withªA delimiter string used to join together the strings obtained from applying <code>map_each</code>, in the same manner as <a href='../core/string.html#join'><code>string.join()</code></a>.(
Û
map_eachcallable⁄A Starlark function accepting a single argument and returning either a string, <code>None</code>, or a list of strings. This function is applied to each item of the depset specified in the <code>values</code> parameter(
∫
uniquify¶If true, duplicate strings derived from <code>values</code> will be omitted. Only the first occurrence of each string will remain. Usually this feature is not needed because depsets already omit duplicates, but it can be useful if <code>map_each</code> emits the same string for multiple items."False
ﬂ
format_joinedM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>yAn optional format string pattern applied to the joined string. The format string must have exactly one '%s' placeholder."None
‚
allow_closure…If true, allows the use of closures in function parameters like <code>map_each</code>. Usually this isn't necessary and it risks retaining large analysis-phase data structures into the execution phase."FalseTemplateDict"Add depset of valuesäAn Args-like structure for use in ctx.actions.expand_template(), which allows for deferring evaluation of values till the execution phase.
€
toolchain_typeG
	mandatorybool"4Whether the toolchain type is mandatory or optional.=
toolchain_typeLabel"$The toolchain type that is required.AA data type describing a dependency on a specific toolchain type.
Ø
ToolchainContextöHolds toolchains available for a particular exec group. Toolchain targets are accessed by indexing with the toolchain type, as in <code>ctx.toolchains["//pkg:my_toolchain_type"]</code>. If the toolchain was optional and no toolchain was resolved, this will return <code>None</code>.
ô

transitionä<p>Represents a configuration transition across a dependency edge. For example, if <code>//package:foo</code> depends on <code>//package:bar</code> with a configuration transition, then the configuration of these two targets will differ: <code>//package:bar</code>'s transition will be determined by that of <code>//package:foo</code>, as subject to the function defined by a transition object.
ñ@
apple_common…
AppleDebugOutputsProvider"©The constructor/key for the <code>AppleDebugOutputs</code> provider.<p>If a target propagates the <code>AppleDebugOutputs</code> provider, use this as the key with which to retrieve it. Example:<br><pre class='language-python'>
dep = ctx.attr.deps[0]
p = dep[apple_common.AppleDebugOutputs]
</pre>Ÿ
AppleDynamicFrameworkProvider"µThe constructor/key for the <code>AppleDynamicFramework</code> provider.<p>If a target propagates the <code>AppleDynamicFramework</code> provider, use this as the key with which to retrieve it. Example:<br><pre class='language-python'>
dep = ctx.attr.deps[0]
p = dep[apple_common.AppleDynamicFramework]
</pre>Ÿ
AppleExecutableBinaryProvider"µThe constructor/key for the <code>AppleExecutableBinary</code> provider.<p>If a target propagates the <code>AppleExecutableBinary</code> provider, use this as the key with which to retrieve it. Example:<br><pre class='language-python'>
dep = ctx.attr.deps[0]
p = dep[apple_common.AppleExecutableBinary]
</pre>ï
ObjcProvider"ÇThe constructor/key for the <code>Objc</code> provider.<p>If a target propagates the <code>Objc</code> provider, use this as the key with which to retrieve it. Example:<br><pre class='language-python'>
dep = ctx.attr.deps[0]
p = dep[apple_common.Objc]
</pre>÷
XcodePropertiesProvider"∏The constructor/key for the <code>XcodeVersionProperties</code> provider.<p>If a target propagates the <code>XcodeVersionProperties</code> provider, use this as the key with which to retrieve it. Example:<br><pre class='language-python'>
dep = ctx.attr.deps[0]
p = dep[apple_common.XcodeVersionProperties]
</pre>e
XcodeVersionConfigProvider"EThe constructor/key for the <code>XcodeVersionConfig</code> provider.ë
apple_host_system_envZ
R
xcode_config@A provider containing information about the xcode configuration.(dict"õReturns a <a href='../core/dict.html'>dict</a> of environment variables that should be set for actions that need to run build tools on an Apple host system, such as the  version of Xcode that should be used. The keys are variable names and the values  are their corresponding values.]
apple_toolchainapple_toolchain"7Utilities for resolving items from the apple toolchain.≥
dotted_versionM
<
version/The string representation of the DottedVersion.(DottedVersion"RCreates a new <a href="../builtins/DottedVersion.html">DottedVersion</a> instance.¶
link_multi_arch_binaryÏ
#
ctxThe Starlark rule context.(
‡

avoid_depsç<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Target.html">Target</a>s; or <code>None</code>ªA list of <code>Target</code>s that are in the dependency graph of the binary but whose libraries should not be linked into the binary. This is the case for dependencies that will be found at runtime in another image, such as the bundle loader or any dynamic libraries/frameworks that will be loaded by this binary."None
º
extra_linkoptss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s1Extra linkopts to be passed to the linker action."[]
∑
extra_link_inputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s)Extra files to pass to the linker action."[]
–
extra_requested_featuress<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s;Extra requested features to be passed to the linker action."[]
Œ
extra_disabled_featuress<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s:Extra disabled features to be passed to the linker action."[]
º
stampÆWhether to include build information in the linked binary. If 1, build information is always included. If 0, build information is always excluded. If -1 (the default), then the behavior is determined by the --[no]stamp flag. This should be set to 0 when generating the executable output for test rules."-1struct"úLinks a (potentially multi-architecture) binary targeting Apple platforms. This method comprises a bulk of the logic of the Starlark <code>apple_binary</code> rule in the rules_apple domain and exists to aid in the migration of its linking logic to Starlark in rules_apple.
<p>This API is <b>highly experimental</b> and subject to change at any time. Do not depend on the stability of this function at this time.¯
link_multi_arch_static_library-
#
ctxThe Starlark rule context.(struct"¶Links a (potentially multi-architecture) static library targeting Apple platforms. This method comprises a part of the Starlark <code>apple_static_library</code> rule logic, in the rules_apple domain and exists to aid in the migration of its linking logic to Starlark in rules_apple.
<p>This API is <b>highly experimental</b> and subject to change at any time. Do not depend on the stability of this function at this time.á
new_dynamic_framework_provider¨
í
binaryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>3The dylib binary artifact of the dynamic framework."None
j
cc_info]A CcInfo which contains information about the transitive dependencies linked into the binary.(
Ñ
framework_dirsç<a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>\The framework path names used as link inputs in order to link against the dynamic framework."None
ä
framework_filesç<a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../builtins/File.html">File</a>s; or <code>None</code>aThe full set of artifacts that should be included as inputs to link against the dynamic framework"NoneAppleDynamicFramework"6Creates a new AppleDynamicFramework provider instance.Í
new_executable_binary_providerã
Ö
binaryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>&The binary artifact of the executable."None
j
cc_info]A CcInfo which contains information about the transitive dependencies linked into the binary.(AppleExecutableBinary":Creates a new AppleExecutableBinaryInfo provider instance.u
new_objc_provider:
*
**kwargsDictionary of arguments."{}8ObjcProvider"$Creates a new ObjcProvider instance.Û
platformstruct"ﬁAn enum-like struct that contains the following fields corresponding to Apple platforms:<br><ul><li><code>ios_device</code></li><li><code>ios_simulator</code></li><li><code>macos</code></li><li><code>tvos_device</code></li><li><code>tvos_simulator</code></li><li><code>visionos_device</code></li><li><code>visionos_simulator</code></li><li><code>watchos_device</code></li><li><code>watchos_simulator</code></li></ul><p>These values can be passed to methods that expect a platform, like <a href='../providers/XcodeVersionConfig.html#sdk_version_for_platform'>XcodeVersionConfig.sdk_version_for_platform</a>.Ã
platform_typestruct"≤An enum-like struct that contains the following fields corresponding to Apple platform types:<br><ul><li><code>ios</code></li><li><code>macos</code></li><li><code>tvos</code></li><li><code>visionos</code></li><li><code>watchos</code></li></ul><p>These values can be passed to methods that expect a platform type, like the 'apple' configuration fragment's <a href='../fragments/apple.html#multi_arch_platform'>multi_arch_platform</a> method.<p>Example:<p><pre class='language-python'>
ctx.fragments.apple.multi_arch_platform(apple_common.platform_type.ios)
</pre>⁄
target_apple_env}
R
xcode_config@A provider containing information about the xcode configuration.(
!
platformThe apple platform.(dict"∆Returns a <code>dict</code> of environment variables that should be set for actions that build targets of the given Apple platform type. For example, this dictionary contains variables that denote the platform name and SDK version with which to build. The keys are variable names and the values are their corresponding values.MFunctions for Starlark to access internals of the apple rule implementations.
¯•
attr
boolû
m
default[A default value to use if no value for this attribute is given when instantiating the rule."False
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"∆Creates a schema for a boolean attribute. The corresponding <a href='../builtins/ctx.html#attr'><code>ctx.attr</code></a> attribute will be of type <a href='../core/bool.html'><code>bool</code></a>.ı
int˜
i
default[A default value to use if no value for this attribute is given when instantiating the rule."0
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
⁄
valuesm<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/int.html">int</a>s]The list of allowed values for the attribute. An error is raised if any other value is given."[]	Attribute"ÛCreates a schema for an integer attribute. The value must be in the signed 32-bit range. The corresponding <a href='../builtins/ctx.html#attr'><code>ctx.attr</code></a> attribute will be of type <a href='../core/int.html'><code>int</code></a>.∑
int_list≈
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
8
allow_empty#True if the attribute can be empty."True
Ÿ
defaultm<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/int.html">int</a>s[A default value to use if no value for this attribute is given when instantiating the rule."[]
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None	Attribute"cCreates a schema for a list-of-integers attribute. Each element must be in the signed 32-bit range.ê*
label„
ÿ
defaultπ<a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/LateBoundDefault.html">LateBoundDefault</a>; or NativeComputedDefault; or <a class="anchor" href="../core/function.html">function</a>; or <code>None</code>äA default value to use if no value for this attribute is given when instantiating the rule.Use a string or the <a href="../builtins/Label.html#Label"><code>Label</code></a> function to specify a default value, for example, <code>attr.label(default = "//a:b")</code>."None
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
Ò

executable€True if the dependency has to be executable. This means the label must refer to an executable file, or to a rule that outputs an executable file. Access the label with <code>ctx.executable.&lt;attribute_name&gt;</code>."False
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
Ú
allow_single_file÷This is similar to <code>allow_files</code>, with the restriction that the label must correspond to a single <a href="../builtins/File.html">File</a>. Access it through <code>ctx.file.&lt;attribute_name&gt;</code>."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
«
	providersµThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element. It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
˚
cfgÌ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>. This parameter is required if <code>executable</code> is True to guard against accidentally building host tools in the target configuration. <code>"target"</code> has no semantic effect, so don't set it when <code>executable</code> is False unless it really helps clarify your intentions."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]	Attribute"†<p>Creates a schema for a label attribute. This is a dependency attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.<p>In addition to ordinary source files, this kind of attribute is often used to refer to a tool -- for example, a compiler. Such tools are considered to be dependencies, just like source files. To avoid requiring users to specify the tool's label every time they use the rule in their BUILD files, you can hard-code the label of a canonical tool as the <code>default</code> value of this attribute. If you also want to prevent users from overriding this default, you can make the attribute private by giving it a name that starts with an underscore. See the <a href='https://bazel.build/extending/rules#private-attributes'>Rules</a> page for more information.ä 
label_keyed_string_dictô
8
allow_empty#True if the attribute can be empty."True
Ω
defaults<a class="anchor" href="../core/dict.html">dict</a>; or <a class="anchor" href="../core/function.html">function</a>∏A default value to use if no value for this attribute is given when instantiating the rule.Use strings or the <a href="../builtins/Label.html#Label"><code>Label</code></a> function to specify default values, for example, <code>attr.label_keyed_string_dict(default = {"//a:b": "value", "//a:c": "string"})</code>."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
«
	providersµThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element. It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
û
flagss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDeprecated, will be removed."[]
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
⁄
cfgÃ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]	Attribute"“<p>Creates a schema for an attribute holding a dictionary, where the keys are labels and the values are strings. This is a dependency attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.±!

label_listº
8
allow_empty#True if the attribute can be empty."True
‡
defaultµ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Label.html">Label</a>s; or <a class="anchor" href="../core/function.html">function</a>òA default value to use if no value for this attribute is given when instantiating the rule.Use strings or the <a href="../builtins/Label.html#Label"><code>Label</code></a> function to specify default values, for example, <code>attr.label_list(default = ["//a:b", "//a:c"])</code>."[]
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
õ
allow_files¡<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>¡Whether <code>File</code> targets are allowed. Can be <code>True</code>, <code>False</code> (default), or a list of file extensions that are allowed (for example, <code>[".cc", ".cpp"]</code>)."None
ü
allow_rulesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>~Which rule targets (name of the classes) are allowed. This is deprecated (kept only for compatibility), use providers instead."None
«
	providersµThe providers that must be given by any dependency appearing in this attribute.<p>The format of this argument is a list of lists of providers -- <code>*Info</code> objects returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a> (or in the case of a legacy provider, its string name). The dependency must return ALL providers mentioned in at least ONE of the inner lists. As a convenience, this argument may also be a single-level list of providers, in which case it is wrapped in an outer list with one element. It is NOT required that the rule of the dependency advertises those providers in its <code>provides</code> parameter, however, it is considered best practice."[]
û
flagss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDeprecated, will be removed."[]
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
⁄
cfgÃ<a href="https://bazel.build/extending/rules#configurations">Configuration</a> of the attribute. It can be either <code>"exec"</code>, which indicates that the dependency is built for the <code>execution platform</code>, or <code>"target"</code>, which indicates that the dependency is build for the <code>target platform</code>. A typical example of the difference is when building mobile apps, where the <code>target platform</code> is <code>Android</code> or <code>iOS</code> while the <code>execution platform</code> is <code>Linux</code>, <code>macOS</code>, or <code>Windows</code>."None
Â
aspectsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s]Aspects that should be applied to the dependency or dependencies specified by this attribute."[]	Attribute"„<p>Creates a schema for a list-of-labels attribute. This is a dependency attribute. The corresponding <a href='../builtins/ctx.html#attr'><code>ctx.attr</code></a> attribute will be of type <a href='../core/list.html'>list</a> of <a href='../builtins/Target.html'><code>Target</code>s</a>.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time (within the rule's implementation function), when retrieving the attribute value from <code>ctx.attr</code>, labels are replaced by the corresponding <a href='../builtins/Target.html'><code>Target</code></a>s. This allows you to access the providers of the current target's dependencies.∏
outputØ
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"˚<p>Creates a schema for an output (label) attribute.</p><p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time, the corresponding <a href='../builtins/File.html'><code>File</code></a> can be retrieved using <a href='../builtins/ctx.html#outputs'><code>ctx.outputs</code></a>.
output_listÈ
8
allow_empty#True if the attribute can be empty."True
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"ÙCreates a schema for a list-of-outputs attribute.<p>This attribute contains unique <a href='../builtins/Label.html'><code>Label</code></a> values. If a string is supplied in place of a <code>Label</code>, it will be converted using the <a href='../builtins/Label.html#Label'>label constructor</a>. The relative parts of the label path, including the (possibly renamed) repository, are resolved with respect to the instantiated target's package.<p>At analysis time, the corresponding <a href='../builtins/File.html'><code>File</code></a> can be retrieved using <a href='../builtins/ctx.html#outputs'><code>ctx.outputs</code></a>.Æ
string“
Ω
defaultQ<a class="anchor" href="../core/string.html">string</a>; or NativeComputedDefault[A default value to use if no value for this attribute is given when instantiating the rule."''
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
‡
valuess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s]The list of allowed values for the attribute. An error is raised if any other value is given."[]	Attribute"OCreates a schema for a <a href='../core/string.html#attr'>string</a> attribute.≈
string_dict’
8
allow_empty#True if the attribute can be empty."True
j
default[A default value to use if no value for this attribute is given when instantiating the rule."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"^Creates a schema for an attribute holding a dictionary, where the keys and values are strings.©
string_listÊ
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False
8
allow_empty#True if the attribute can be empty."True
˙
defaultç<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or NativeComputedDefault[A default value to use if no value for this attribute is given when instantiating the rule."[]
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None	Attribute"1Creates a schema for a list-of-strings attribute.„
string_list_dict’
8
allow_empty#True if the attribute can be empty."True
j
default[A default value to use if no value for this attribute is given when instantiating the rule."{}
≥
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>WA description of the attribute that can be extracted by documentation generating tools."None
l
	mandatoryXIf true, the value must be specified explicitly (even if it has a <code>default</code>)."False	Attribute"wCreates a schema for an attribute holding a dictionary, where the keys are strings and the values are lists of strings.ΩThis is a top-level module for defining the attribute schemas of a rule or aspect. Each function returns an object representing the schema of a single attribute. These objects are used as the values of the <code>attrs</code> dictionary argument of <a href="../globals/bzl.html#rule"><code>rule()</code></a> and <a href="../globals/bzl.html#aspect"><code>aspect()</code></a>.<p>See the Rules page for more on <a href='https://bazel.build/extending/rules#attributes'>defining</a> and <a href='https://bazel.build/extending/rules#implementation_function'>using</a> attributes.
Ì¥
	cc_common
CcToolchainInfoProvider"bThe key used to retrieve the provider that contains information about the C++ toolchain being used◊
action_is_enabledt
?
feature_configuration$Feature configuration to be queried.(
+
action_nameName of the action_config.(bool"LReturns True if given action_config is enabled in the feature configuration.Â
configure_features˙
k
ctxK<a class="anchor" href="../builtins/ctx.html">ctx</a>; or <code>None</code>The rule context."None
?
cc_toolchain-cc_toolchain for which we configure features.(
†
languageM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>?The language to configure for: either c++ or objc (default c++)"None
9
requested_featuresList of features to be enabled."[]
V
unsupported_features:List of features that are unsupported by the current rule."[]FeatureConfiguration"RCreates a feature_configuration instance. Requires the cpp configuration fragment.ó1
create_cc_toolchain_config_infoº0

ctxThe rule context.(
á
featuresˆContains all flag specifications for one feature.<p>Arguments:</p><p><code>name</code>: The feature's name. It is possible to introduce a feature without a change to Bazel by adding a 'feature' section to the toolchain and adding the corresponding string as feature in the <code>BUILD</code> file.</p><p><code>enabled</code>: If 'True', this feature is enabled unless a rule type explicitly marks it as unsupported.</p><p><code>flag_sets</code>: A FlagSet list. If the given feature is enabled, the flag sets will be applied for the actions are specified for. </p><p><code>env_sets</code>: an EnvSet list. If the given feature is enabled, the env sets will be applied for the actions they are specified for. </p><p><code>requires</code>: A list of feature sets defining when this feature is supported by the  toolchain. The feature is supported if any of the feature sets fully apply, that is, when all features of a feature set are enabled. If <code>requires</code> is omitted, the feature is supported independently of which other features are enabled. Use this for example to filter flags depending on the build mode enabled (opt / fastbuild / dbg). </p><p><code>implies</code>: A string list of features or action configs that are automatically enabled when this feature is enabled. If any of the implied features or action configs cannot be enabled, this feature will (silently) not be enabled either. </p><p><code>provides</code>: A list of names this feature conflicts with. </p>A feature cannot be enabled if:</br>- <code>provides</code> contains the name of a different feature or action config that we want to enable.</br>- <code>provides</code> contains the same value as a 'provides' in a different feature or action config that we want to enable. Use this in order to ensure that incompatible features cannot be accidentally activated at the same time, leading to hard to diagnose compiler errors."[]
„	
action_configsÃ	An action config corresponds to a Bazel action, and allows selection of a tool based on activated features. Action config activation occurs by the same semantics as features: a feature can 'require' or 'imply' an action config in the same way that it would another feature.<p>Arguments:</p><p><code>action_name</code>: The name of the Bazel action that this config applies to, e.g. 'c-compile' or 'c-module-compile'.</p><p><code>enabled</code>: If 'True', this action is enabled unless a rule type explicitly marks it as unsupported.</p><p><code>tools</code>: The tool applied to the action will be the first tool with a feature set that matches the feature configuration.  An error will be thrown if no tool matches a provided feature configuration - for that reason, it's a good idea to provide a default tool with an empty feature set.</p><p><code>flag_sets</code>: If the given action config is enabled, the flag sets will be applied to the corresponding action.</p><p><code>implies</code>: A list of features or action configs that are automatically enabled when this action config is enabled. If any of the implied features or action configs cannot be enabled, this action config will (silently) not be enabled either.</p>"[]
á
artifact_name_patternsËThe name for an artifact of a given category of input or output artifacts to an action.<p>Arguments:</p><p><code>category_name</code>: The category of artifacts that this selection applies to. This field is compared against a list of categories defined in Bazel. Example categories include "linked_output" or the artifact for this selection. Together with the extension it is used to create an artifact name based on the target name.</p><p><code>extension</code>: The extension for creating the artifact for this selection. Together with the prefix it is used to create an artifact name based on the target name.</p>"[]
ÿ
cxx_builtin_include_directories∞<p>Built-in include directories for C++ compilation. These should be the exact paths used by the compiler, and are generally relative to the exec root.</p><p>The paths used by the compiler can be determined by 'gcc -E -xc++ - -v'.</p><p>We currently use the C++ paths also for C compilation, which is safe as long as there are no name clashes between C++ and C header files.</p><p>Relative paths are resolved relative to the configuration file directory.</p><p>If the compiler has --sysroot support, then these paths should use %sysroot% rather than the include path, and specify the sysroot attribute in order to give blaze the information necessary to make the correct replacements.</p>"[]
‰
toolchain_identifier…<p>The unique identifier of the toolchain within the crosstool release. It must be possible to use this as a directory name in a path.</p><p>It has to match the following regex: [a-zA-Z_][\.\- \w]*</p>(
q
host_system_nameM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>Ignored."None
,
target_system_nameThe GNU System Name.(
/

target_cpuThe target architecture string.(
>
target_libc-The libc version string (e.g. "glibc-2.2.2").(
=
compiler/The compiler version string (e.g. "gcc-4.1.1").(
õ
abi_versionM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>7The abi in use, which is a gcc version. E.g.: "gcc-3.4""None
ó
abi_libc_versionM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>.The glibc version used by the abi we're using."None
˜

tool_paths‰Tool locations.<p>Arguments:</p><p><code>name</code>: Name of the tool.</p><p><code>path</code>: Location of the tool; Can be absolute path (in case of non hermetic toolchain), or path relative to the cc_toolchain's package.</p>"[]
G
make_variables1A make variable that is made accessible to rules."[]
Ù
builtin_sysrootM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>ãThe built-in sysroot. If this attribute is not present, Bazel does not allow using a different sysroot, i.e. through the --grte_top option."None
á
cc_target_osM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>"Internal purpose only, do not use."NoneCcToolchainConfigInfo"5Creates a <code>CcToolchainConfigInfo</code> provider»
create_compilation_context˝
@
headers,Set of headers needed to compile this target"unbound
ﬁ
system_includes¡Set of search paths for header files referenced by angle brackets, i.e. #include &lt;foo/bar/header.h&gt;. They can be either relative to the exec root or absolute. Usually passed with -isystem"unbound
|
includesgSet of search paths for header files referenced both by angle bracket and quotes.Usually passed with -I"unbound
Œ
quote_includes≤Set of search paths for header files referenced by quotes, i.e. #include "foo/bar/header.h". They can be either relative to the exec root or absolute. Usually passed with -iquote"unbound
c
framework_includesDSet of framework search paths for header files (Apple platform only)"unbound
Å
definesmSet of defines needed to compile this target. Each define is a string. Propagated transitively to dependents."unbound
ã
local_definesqSet of defines needed to compile this target. Each define is a string. Not propagated transitively to dependents."unboundCompilationContext"*Creates a <code>CompilationContext</code>.ÿ
create_compilation_outputsï
y
objectsQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>List of object files."None
Å
pic_objectsQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>List of pic object files."NoneCcCompilationOutputs""Create compilation outputs object.à
create_compile_variables∫
I
cc_toolchain7cc_toolchain for which we are creating build variables.(
?
feature_configuration$Feature configuration to be queried.(
∏
source_file¢Optional source file for the compilation. Please prefer passing source_file here over appending it to the end of the command line generated from cc_common.get_memory_inefficient_command_line, as then it's in the power of the toolchain author to properly specify and position compiler flags."None
∑
output_file°Optional output file of the compilation. Please prefer passing output_file here over appending it to the end of the command line generated from cc_common.get_memory_inefficient_command_line, as then it's in the power of the toolchain author to properly specify and position compiler flags."None
’
user_compile_flagsâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>-List of additional compilation flags (copts)."None
é
include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>Depset of include directories."None
ö
quote_include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>$Depset of quote include directories."None
ú
system_include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>%Depset of system include directories."None
¢
framework_include_directoriesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>(Depset of framework include directories."None
ê
preprocessor_definesQ<a class="anchor" href="../builtins/depset.html">depset</a>; or <code>None</code>Depset of preprocessor defines."None
z
thinlto_indexM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>LTO index file path."None
ù
thinlto_input_bitcode_fileM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>*Bitcode file that is input to LTO backend."None
ù
thinlto_output_object_fileM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>*Object file that is output by LTO backend."None
T
use_picBWhen true the compilation will generate position independent code."False
(
add_legacy_cxx_optionsUnused."False
í
variables_extension3<a class="anchor" href="../core/dict.html">dict</a>=A dictionary of additional variables used by compile actions."unbound	Variables"/Returns variables used for compilation actions.Å
create_library_to_link¬
)
actions<code>actions</code> object.(
P
feature_configuration1<code>feature_configuration</code> to be queried."None
G
cc_toolchain1<code>CcToolchainInfo</code> provider to be used."None
ò
static_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>1<code>File</code> of static library to be linked."None
†
pic_static_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>5<code>File</code> of pic static library to be linked."None
˘
dynamic_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>ê<code>File</code> of dynamic library to be linked. Always used for runtime and used for linking if <code>interface_library</code> is not passed."None
û
interface_libraryM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>4<code>File</code> of interface library to be linked."None
•
pic_objectss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>sExperimental, do not use"unbound
°
objectss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>sExperimental, do not use"unbound
]

alwayslinkHWhether to link the static library/objects in the --whole_archive block."False
À
dynamic_library_symlink_path7<a class="anchor" href="../core/string.html">string</a>nOverride the default path of the dynamic library link in the solib directory. Empty string to use the default."''
ñ
interface_library_symlink_pathpOverride the default path of the interface library link in the solib directory. Empty string to use the default."''LibraryToLink""Creates <code>LibraryToLink</code>∏
create_link_variablesÏ
I
cc_toolchain7cc_toolchain for which we are creating build variables.(
?
feature_configuration$Feature configuration to be queried.(
ø
library_search_directoriesQ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a>HDepset of directories where linker will look for libraries at link time."None
≈
"runtime_library_search_directoriesQ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a>FDepset of directories where loader will look for libraries at runtime."None
ë
user_link_flagsM<code>None</code>; or <a class="anchor" href="../core/list.html">sequence</a>)List of additional link flags (linkopts)."None
/
output_fileOptional output file path."None
-

param_fileOptional param file path."None
*
def_fileOptional .def file path."None
Ö
is_using_linkerÎTrue when using linker, False when archiver. Caller is responsible for keeping this in sync with action name used (is_using_linker = True for linking executable or dynamic library, is_using_linker = False for archiving static library)."True
Ì
is_linking_dynamic_library«True when creating dynamic library, False when executable or static library. Caller is responsible for keeping this in sync with action name used. This field will be removed once b/65151735 is fixed."False
≥
must_keep_debugôWhen set to False, bazel will expose 'strip_debug_symbols' variable, which is usually used to use the linker to strip debug symbols from the output file."True
R
use_test_only_flags4When set to true, 'is_cc_test' variable will be set."False
'
is_static_linking_modeUnused."True	Variables"0Returns link variables used for linking actions.ƒ
create_linker_inputá
N
ownerCThe label of the target that produced all files used in this input.(
â
	librariesQ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a>#List of <code>LibraryToLink</code>."None
Á
user_link_flagsÖ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s≈User link flags passed as strings. Accepts either [String], [[String]] or depset(String). The latter is discouraged as it's only kept for compatibility purposes, the depset is flattened. If you want to propagate user_link_flags via unflattened depsets() wrap them in a LinkerInput so that they are not flattened till the end."None
±
additional_inputsQ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a>CFor additional inputs to the linking action, e.g.: linking scripts."NoneLinkerInput"#Creates a <code>LinkerInput</code>.Ÿ
create_linking_contextñ
ç
linker_inputsQ<code>None</code>; or <a class="anchor" href="../builtins/depset.html">depset</a>#Depset of <code>LinkerInput</code>."None
ò
libraries_to_linkM<code>None</code>; or <a class="anchor" href="../core/list.html">sequence</a>≠<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>--+incompatible_require_linker_input_cc_api</code>. Use this flag to verify your code is compatible with its imminent removal. <br>List of <code>LibraryToLink</code>."None
ù
user_link_flagsM<code>None</code>; or <a class="anchor" href="../core/list.html">sequence</a>¥<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>--+incompatible_require_linker_input_cc_api</code>. Use this flag to verify your code is compatible with its imminent removal. <br>List of user link flags passed as strings."None
∏
additional_inputsM<code>None</code>; or <a class="anchor" href="../core/list.html">sequence</a>Õ<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>--+incompatible_require_linker_input_cc_api</code>. Use this flag to verify your code is compatible with its imminent removal. <br>For additional inputs to the linking action, e.g.: linking scripts."NoneLinkingContext"&Creates a <code>LinkingContext</code>.Ø

/create_linking_context_from_compilation_outputs€
)
actions<code>actions</code> object.(
L
feature_configuration1<code>feature_configuration</code> to be queried.(
C
cc_toolchain1<code>CcToolchainInfo</code> provider to be used.(
M
compilation_outputs4Compilation outputs containing object files to link.(
:
user_link_flags#Additional list of linking options."[]
û
linking_contextsÖLibraries from dependencies. These libraries will be linked into the output artifact of the link() call, be it a binary or a library."[]
Y
nameOThis is used for naming the output artifacts of actions created by this method.(
I
language6Only C++ supported for now. Do not use this parameter."'c++'
B

alwayslink-Whether this library should always be linked."False
\
additional_inputsCFor additional inputs to the linking action, e.g.: linking scripts."[]
O
disallow_static_libraries+Whether static libraries should be created."False
O
disallow_dynamic_library,Whether a dynamic library should be created."Falsetuple"ùShould be used for creating library rules that can propagate information downstream in order to be linked later by a top level rule that does transitive linking to create an executable or dynamic library. Returns tuple of (<code>CcLinkingContext</code>, <code>CcLinkingOutputs</code>).ﬂ
%do_not_use_tools_cpp_compiler_presentNoneType"´Do not use this field, its only purpose is to help with migration from config_setting.values{'compiler') to config_settings.flag_values{'@bazel_tools//tools/cpp:compiler'}≥
get_environment_variables⁄
?
feature_configuration$Feature configuration to be queried.(
Õ
action_nameªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(
A
	variables2Build variables to be used for template expansion.(dict"9Returns environment variables to be set for given action.Ï
get_execution_requirementsõ
?
feature_configuration$Feature configuration to be queried.(
Õ
action_nameªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(sequence"0Returns execution requirements for given action.Ä
#get_memory_inefficient_command_lineﬂ
?
feature_configuration$Feature configuration to be queried.(
Õ
action_nameªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(
B
	variables3Build variables to be used for template expansions.(sequence"ˆReturns flattened command line flags for given action, using given variables for expansion. Flattens nested sets and ideally should not be used, or at least should not outlive analysis. Work on memory efficient function returning Args is ongoing.÷
get_tool_for_actionô
?
feature_configuration$Feature configuration to be queried.(
Õ
action_nameªName of the action. Has to be one of the names in @bazel_tools//tools/build_defs/cc:action_names.bzl (https://github.com/bazelbuild/bazel/blob/master/tools/build_defs/cc/action_names.bzl)(string"#Returns tool path for given action.≈

is_enabledo
?
feature_configuration$Feature configuration to be queried.(
&
feature_nameName of the feature.(bool"FReturns True if given feature is enabled in the feature configuration.∫
linkÖ
)
actions<code>actions</code> object.(
L
feature_configuration1<code>feature_configuration</code> to be queried.(
C
cc_toolchain1<code>CcToolchainInfo</code> provider to be used.(
¿
compilation_outputsm<a class="anchor" href="../builtins/CcCompilationOutputs.html">CcCompilationOutputs</a>; or <code>None</code>4Compilation outputs containing object files to link."None
9
user_link_flags"Additional list of linker options."[]
x
linking_contexts`Linking contexts from dependencies to be linked into the linking context generated by this rule."[]
Y
nameOThis is used for naming the output artifacts of actions created by this method.(
I
language6Only C++ supported for now. Do not use this parameter."'c++'
M
output_type0Can be either 'executable' or 'dynamic_library'."'executable'
W
link_deps_statically9 True to link dependencies statically, False dynamically."True
˚
stampÓWhether to include build information in the linked executable, if output_type is 'executable'. If 1, build information is always included. If 0 (the default build information is always excluded. If -1, uses the default behavior, which may be overridden by the --[no]stamp flag. This should be unset (or set to 0) when generating the executable output for test rules."0
’
additional_inputsw<a class="anchor" href="../core/list.html">sequence</a>; or <a class="anchor" href="../builtins/depset.html">depset</a>CFor additional inputs to the linking action, e.g.: linking scripts."[]
ñ
additional_outputs7<a class="anchor" href="../core/list.html">sequence</a>>For additional outputs to the linking action, e.g.: map files."unboundCcLinkingOutputs"*Should be used for C++ transitive linking.£
merge_compilation_contexts«
∞
compilation_contextsìList of <code>CompilationContexts</code>s to be merged. The headers of each context will be exported by the direct fields in the returned provider."[]CompilationContext";Merges multiple <code>CompilationContexts</code>s into one.j
merge_compilation_outputs1

compilation_outputs"[]CcCompilationOutputs"Merge compilation outputs.DUtilities for C++ compilation, linking, and command line generation.
∑
configÖ
boola
Q
flagBWhether or not this build setting is callable on the command line."FalseBuildSetting"A bool-typed build settingŒ
exec£
â

exec_groupM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>•The name of the exec group whose execution platform this transition will use. If not provided, this exec transition will use the target's default execution platform."NoneExecTransitionFactory" Creates an execution transition.à
inta
Q
flagBWhether or not this build setting is callable on the command line."FalseBuildSetting"An integer-typed build settingÔ
string∆
Q
flagBWhether or not this build setting is callable on the command line."False
‚
allow_multiple»Deprecated, use a <code>string_list</code> setting with <code>repeatable = True</code> instead. If set, this flag is allowed to be set multiple times on the command line. The Value of the flag as accessed in transitions and build setting implementation function will be a list of strings. Insertion order and repeated values are both maintained. This list can be post-processed in the build setting implementation function if different behavior is desired."FalseBuildSetting"A string-typed build settingÑ
string_listÈ
Q
flagBWhether or not this build setting is callable on the command line."False
Ö

repeatableÔIf set, instead of expecting a comma-separated value, this flag is allowed to be set multiple times on the command line with each individual value treated as a single string to add to the list value. Insertion order and repeated values are both maintained. This list can be post-processed in the build setting implementation function if different behavior is desired."FalseBuildSetting"àA string list-typed build setting. On the command line pass a list using comma-separated value like <code>--//my/setting=foo,bar</code>.œThis is a top-level module for creating configuration transitions and build setting descriptors which describe what kind of build setting (if any) a rule is. <p>ex: the following rule is marked as a build setting by setting the <code>build_setting</code> parameter of the <code>rule()</code> function. Specifically it is a build setting of type <code>int</code> and is a <code>flag</code> which means this build setting is callable on the command line.<br><pre class=language-python>  my_rule = rule(
    implementation = _impl,
    build_setting = config.int(flag = True),
    ...
  )</pre>
¿
config_commonj
FeatureFlagInfoProvider"MThe key used to retrieve the provider containing config_feature_flag's value.˚
toolchain_type∂
•
nameu<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>$The toolchain type that is required.(
|
	mandatory3<a class="anchor" href="../core/bool.html">bool</a>4Whether the toolchain type is mandatory or optional."Truetoolchain_type"0Declare a rule's dependency on a toolchain type.EFunctions for Starlark to interact with Blaze's configurability APIs.
Ä

coverage_commonØ	
instrumented_files_info∆

ctxThe rule context.(
e
source_attributesLA list of attribute names which contain source files processed by this rule."[]
á
dependency_attributesjA list of attribute names which might provide runtime dependencies (either code dependencies or runfiles)."[]
®

extensionsâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>áFile extensions used to filter files from source_attributes. For example, 'js'. If not provided (or None), then all files from source_attributes will be added to instrumented files, if an empty list is provided, then no files from source attributes will be added."None
Ù
metadata_filess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>siAdditional files required to generate coverage LCOV files after code execution. e.g. .gcno files for C++."[]InstrumentedFilesInfo" Creates a new <a class="anchor" href="../providers/InstrumentedFilesInfo.html">InstrumentedFilesInfo</a> instance. Use this provider to communicate coverage-related attributes of the current build rule.;Helper functions to access coverage-related infrastructure.
¿H
java_commonT
BootClassPathInfoProvider"5The provider used to supply bootclasspath information
JavaRuntimeInfoProvider"bThe key used to retrieve the provider that contains information about the Java runtime being used.É
JavaToolchainInfoProvider"dThe key used to retrieve the provider that contains information about the Java toolchain being used.Ú)
compileî(

ctxThe rule context.(
Î
source_jarss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>scA list of the jars to be compiled. At least one of source_jars or source_files should be specified."[]
˘
source_filess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>spA list of the Java source files to be compiled. At least one of source_jars or source_files should be specified."[]


output(
∑
output_source_jarM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>MThe output source jar. Optional. Defaults to `{output_jar}-src.jar` if unset."None
µ

javac_optss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s.A list of the desired javac options. Optional."[]
¶
depsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>s!A list of dependencies. Optional."[]
∂
runtime_depsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>s)A list of runtime dependencies. Optional."[]
§
exportsw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sA list of exports. Optional."[]
°
pluginsÛ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>s; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sA list of plugins. Optional."[]
≥
exported_pluginsÛ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>s; or <a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>s%A list of exported plugins. Optional."[]
“
native_librariesx<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../providers/CcInfo.html">CcInfo</a>s@CC native library dependencies that are needed for this library."[]
ô
&annotation_processor_additional_inputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>svA list of inputs that the Java compilation action will take in addition to the Java sources for annotation processing."[]
õ
'annotation_processor_additional_outputss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>swA list of outputs that the Java compilation action will output in addition to the class jar from annotation processing."[]
’
strict_depsºA string that specifies how to handle strict deps. Possible values: 'OFF', 'ERROR', 'WARN' and 'DEFAULT'. For more details see /docs/user-manual#flag--strict_java_deps. By default 'ERROR'."'ERROR'
S
java_toolchain?A JavaToolchainInfo to be used for this compilation. Mandatory.(
´
bootclasspathìA BootClassPathInfo to be used for this compilation. If present, overrides the bootclasspath associated with the provided java_toolchain. Optional."None
Ù
host_javabase‹<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>--+incompatible_java_common_parameters</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Deprecated: You can drop this parameter (host_javabase is provided with java_toolchain)"None
Ö

sourcepaths<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]
Ñ
	resourcess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]
à
resource_jarss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]
é
classpath_resourcess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s"[]

	neverlink"False
æ
enable_annotation_processingóDisables annotation processing in this compilation, causing any annotation processors provided in plugins or in exported_plugins of deps to be ignored."True
ò
enable_compile_jar_actionÙEnables header compilation or ijar creation. If set to False, it forces use of the full class jar in the compilation classpaths of any dependants. Doing so is intended for use by non-library targets such as binaries that do not have dependants."True
Ã
add_exportss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sDAllow this library to access the given <module>/<package>. Optional."[]
◊
	add_openss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sQAllow this library to reflectively access the given <module>/<package>. Optional."[]struct"œCompiles Java source files/jars from the implementation of a Starlark rule and returns a provider that represents the results of the compilation and can be added to the set of providers emitted by this rule.
merge≤
ß
	providersw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/struct.html">struct</a>sThe list of providers to merge.(struct"2Merges the given providers into a single JavaInfo.º
pack_sourcesù

actionsctx.actions(
≤

output_jarM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>Œ<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>--+incompatible_java_common_parameters</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Deprecated: The output jar of the rule. Used to name the resulting source jar. The parameter sets output_source_jar parameter to `{output_jar}-src.jar`.Use output_source_jar parameter directly instead."None
Ä
output_source_jarM<a class="anchor" href="../builtins/File.html">File</a>; or <code>None</code>The output source jar."None
¡
sourcess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s=A list of Java source files to be packed into the source jar."[]
ø
source_jarss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/File.html">File</a>s7A list of source jars to be packed into the source jar."[]
F
java_toolchain2A JavaToolchainInfo to used to find the ijar tool.(
Ù
host_javabase‹<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>--+incompatible_java_common_parameters</code>. Use this flag to verify your code is compatible with its imminent removal. <br>Deprecated: You can drop this parameter (host_javabase is provided with java_toolchain)"NoneFile"ãPacks sources and source jars into a single source jar file. The return value is typically passed to<p><code><a class="anchor" href="../providers/JavaInfo.html">JavaInfo</a>#source_jar</code></p>.At least one of parameters output_jar or output_source_jar is required.Ì
run_ijar¢

actionsctx.actions(
 
jarThe jar to run ijar on.(
ï
target_labelO<a class="anchor" href="../builtins/Label.html">Label</a>; or <code>None</code>≠A target label to stamp the jar with. Used for <code>add_dep</code> support. Typically, you would pass <code>ctx.label</code> to stamp the jar with the current rule's label."None
F
java_toolchain2A JavaToolchainInfo to used to find the ijar tool.(File"ªRuns ijar on a jar, stripping it of its method bodies. This helps reduce rebuilding of dependent jars during any recompiles consisting only of simple changes to method implementations. The return value is typically passed to <code><a class="anchor" href="../providers/JavaInfo.html">JavaInfo</a>#compile_jar</code>.á
	stamp_jar◊

actionsctx.actions(
%
jarThe jar to run stamp_jar on.(
¿
target_label≠A target label to stamp the jar with. Used for <code>add_dep</code> support. Typically, you would pass <code>ctx.label</code> to stamp the jar with the current rule's label.(
K
java_toolchain7A JavaToolchainInfo to used to find the stamp_jar tool.(File"üStamps a jar with a target label for <code>add_dep</code> support. The return value is typically passed to <code><a class="anchor" href="../providers/JavaInfo.html">JavaInfo</a>#compile_jar</code>. Prefer to use <code><a class="anchor" href="#run_ijar">run_ijar</a></code> when possible.3Utilities for Java compilation support in Starlark.
∏ù
nativeë
existing_rule,
!
nameThe name of the target.(unknown"œReturns an immutable dict-like object that describes the attributes of a rule instantiated in this thread's package, or <code>None</code> if no rule instance of that name exists.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>If the <code>--noincompatible_existing_rules_immutable_view</code> flag is set, instead returns a new mutable dict with the same content.<p>The result contains an entry for each attribute, with the exception of private ones (whose names do not start with a letter) and a few unrepresentable legacy attribute types. In addition, the dict contains entries for the rule instance's <code>name</code> and <code>kind</code> (for example, <code>'cc_binary'</code>).<p>The values of the result represent attribute values as follows:<ul><li>Attributes of type str, int, and bool are represented as is.</li><li>Labels are converted to strings of the form <code>':foo'</code> for targets in the same package or <code>'//pkg:name'</code> for targets in a different package.</li><li>Lists are represented as tuples, and dicts are converted to new, mutable dicts. Their elements are recursively converted in the same fashion.</li><li><code>select</code> values are returned with their contents transformed as described above.</li><li>Attributes for which no value was specified during rule instantiation and whose default value is computed are excluded from the result. (Computed defaults cannot be computed until the analysis phase.).</li></ul><p>If possible, avoid using this function. It makes BUILD files brittle and order-dependent. Also, beware that it differs subtly from the two other conversions of rule attribute values from internal form to Starlark: one used by computed defaults, the other used by <code>ctx.attr.foo</code>.(Â
existing_rules	unknown"≈Returns an immutable dict-like object describing the rules so far instantiated in this thread's package. Each entry of the dict-like object maps the name of the rule instance to the result that would be returned by <code>existing_rule(name)</code>.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>If the <code>--noincompatible_existing_rules_immutable_view</code> flag is set, instead returns a new mutable dict with the same content.<p><em>Note: If possible, avoid using this function. It makes BUILD files brittle and order-dependent. Furthermore, if the </em><code>--noincompatible_existing_rules_immutable_view</code><em> flag is set, this function may be very expensive, especially if called within a loop.</em>(„
exports_filesı
õ
srcss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sThe list of files to export.(
ê

visibilityM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>¨A visibility declaration can to be specified. The files will be visible to the targets specified. If no visibility is specified, the files will be visible to every package."None
∑
licensesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>Licenses to be specified."NoneNoneType"XSpecifies a list of files belonging to this package that are exported to other packages.(ﬂ
globØ
©
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to include."[]
©
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to exclude."[]
G
exclude_directories-A flag whether to exclude directories or not."1
Å
allow_emptyËWhether we allow glob patterns to match nothing. If `allow_empty` is False, each individual include pattern must match something and also the final result must be non-empty (after the matches of the `exclude` patterns are excluded)."unboundsequence"¢Glob returns a new, mutable, sorted list of every file in the current package that:<ul>
<li>Matches at least one pattern in <code>include</code>.</li>
<li>Does not match any of the patterns in <code>exclude</code> (default <code>[]</code>).</li></ul>
If the <code>exclude_directories</code> argument is enabled (set to <code>1</code>), files of type directory will be omitted from the results (default <code>1</code>).(˛
module_namestring"‚The name of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the name of the module hosting the extension. It's the same as the <code>module.name</code> field seen in <code>module_ctx.modules</code>.(ä
module_versionstring"ÎThe version of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the version of the module hosting the extension. It's the same as the <code>module.version</code> field seen in <code>module_ctx.modules</code>.(À
package_group®
(
nameThe unique name for this rule.(
∂
packagess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s1A complete enumeration of packages in this group."[]
∏
includess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s3Other package groups that are included in this one."[]NoneType"åThis function defines a set of packages and assigns a label to the group. The label can be referenced in <code>visibility</code> attributes.(–
package_namestring"≥The name of the package being evaluated, without the repository name. For example, in the BUILD file <code>some/package/BUILD</code>, its value will be <code>some/package</code>. If the BUILD file calls a function defined in a .bzl file, <code>package_name()</code> will match the caller BUILD file package.(π
package_relative_labelÂ
€
inputu<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>YThe input label string or Label object. If a Label object is passed, it's returned as is.(Label"¥Converts the input string into a <a href='../builtins/Label.html'>Label</a> object, in the context of the package currently being initialized (that is, the <code>BUILD</code> file for which the current macro is executing). If the input is already a <code>Label</code>, it is returned unchanged.<p>This function may only be called while evaluating a BUILD file and the macros it directly or indirectly calls; it may not be called in (for instance) a rule implementation function. <p>The result of this function is the same <code>Label</code> value as would be produced by passing the given string to a label-valued attribute of a target declared in the BUILD file. <p><i>Usage note:</i> The difference between this function and <a href='../builtins/Label.html#Label'>Label()</a></code> is that <code>Label()</code> uses the context of the package of the <code>.bzl</code> file that called it, not the package of the <code>BUILD</code> file. Use <code>Label()</code> when you need to refer to a fixed target that is hardcoded into the macro, such as a compiler. Use <code>package_relative_label()</code> when you need to normalize a label string supplied by the BUILD file to a <code>Label</code> object. (There is no way to convert a string to a <code>Label</code> in the context of a package other than the BUILD file or the calling .bzl file. For that reason, outer macros should always prefer to pass Label objects to inner macros rather than label strings.)(â
	repo_namestring"pThe canonical name of the repository containing the package currently being evaluated, with no leading at-signs.(±
repository_namestring"ë<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> Prefer to use <a href="#repo_name"><code>repo_name</code></a> instead, which doesn't contain the spurious leading at-sign, but behaves identically otherwise.<p>The canonical name of the repository containing the package currently being evaluated, with a single at-sign (<code>@</code>) prefixed. For example, in packages that are called into existence by the WORKSPACE stanza <code>local_repository(name='local', path=...)</code> it will be set to <code>@local</code>. In packages in the main repository, it will be set to <code>@</code>.(Á
subpackagesÇ
ª
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s9The list of glob patterns to include in subpackages scan.(
ø
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s;The list of glob patterns to exclude from subpackages scan."[]
ı
allow_emptyﬁWhether we fail if the call returns an empty list. By default empty list indicates potential error in BUILD file where the call to subpackages() is superflous.  Setting to true allows this function to succeed in that case."Falsesequence"–Returns a new mutable list of every direct subpackage of the current package, regardless of file-system directory depth. List returned is sorted and contains the names of subpackages relative to the current package. It is advised to prefer using the methods in bazel_skylib.subpackages module rather than calling this function directly.(—ù
	cc_binary¿ù

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

M
additional_linker_inputs-List of <a href="/concepts/labels">labels</a>"[]
´
args¢<p>
  List of strings; subject to
  <a href="${link make-variables#predefined_label_variables}">$(location)</a> and
  <a href="${link make-variables}">"Make variable"</a> substitution, and
  <a href="#sh-tokenization">Bourne shell tokenization</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
Command line arguments that Bazel will pass to the target when it is executed
either by the <code>run</code> command or as a test. These arguments are
passed before the ones that are specified on the <code>bazel run</code> or
<code>bazel test</code> command line.
</p>

<p>
<em class="harmful">NOTE: The arguments are not passed when you run the target
outside of Bazel (for example, by manually executing the binary in
<code>bazel-bin/</code>).</em>
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>


coptsList of strings"[]

definesList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

A
dynamic_deps-List of <a href="/concepts/labels">labels</a>"[]
Á
envﬂ<p>Dictionary of strings; values are subject to
<a href="${link make-variables#predefined_label_variables}">$(location)</a> and
<a href="${link make-variables}">"Make variable"</a> substitution; default is <code>{}</code></p>

<p>Specifies additional environment variables to set when the target is
  executed by <code>bazel run</code>.
</p>

<p>
  This attribute only applies to native rules, like <code>cc_binary</code>, <code>py_binary</code>,
  and <code>sh_binary</code>.  It does not apply to Starlark-defined executable rules.
</p>

<p>
<em class="harmful">NOTE: The environment variables are not set when you run the target
outside of Bazel (for example, by manually executing the binary in
<code>bazel-bin/</code>).</em>
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>




hdrs_checkString"""

includesList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

`
link_extra_lib$<a href="/concepts/labels">Label</a>"("@bazel_tools//tools/cpp:link_extra_lib"

linkoptsList of strings"[]


linksharedBoolean"False


linkstaticBoolean"True
$
local_definesList of strings"[]
P
malloc$<a href="/concepts/labels">Label</a>" "@bazel_tools//tools/cpp:malloc"

nocoptsString"""
Ê
output_licenses“<p>List of strings; default is <code>[]</code></p>

<p>
The licenses of the output files that this binary generates.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


B
reexport_deps-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>


stampInteger"-1
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

:
win_def_file$<a href="/concepts/labels">Label</a>"None(¬à
	cc_import±à

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



includesList of strings"[]
?
interface_library$<a href="/concepts/labels">Label</a>"None

linkoptsList of strings"[]
<
objects-List of <a href="/concepts/labels">labels</a>"[]
@
pic_objects-List of <a href="/concepts/labels">labels</a>"[]
@
pic_static_library$<a href="/concepts/labels">Label</a>"None
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

<
shared_library$<a href="/concepts/labels">Label</a>"None
<
static_library$<a href="/concepts/labels">Label</a>"None
!
system_providedBoolean"False
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(Ôê

cc_library›ê

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]
O
additional_compiler_inputs-List of <a href="/concepts/labels">labels</a>"[]
M
additional_linker_inputs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>


coptsList of strings"[]

definesList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


à

hdrs_checkString"r<unknown object com.google.devtools.build.lib.rules.cpp.CcStarlarkInternal$DefaultHdrsCheckBuiltinComputedDefault>
H
implementation_deps-List of <a href="/concepts/labels">labels</a>"[]

include_prefixString"""

includesList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


linkoptsList of strings"[]
7
	linkstamp$<a href="/concepts/labels">Label</a>"None


linkstaticBoolean"False
$
local_definesList of strings"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

"
strip_include_prefixString"""
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

A
textual_hdrs-List of <a href="/concepts/labels">labels</a>"[]
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

:
win_def_file$<a href="/concepts/labels">Label</a>"None(ë{
cc_proto_library˙z

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(˝~
cc_shared_libraryÂ~

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

M
additional_linker_inputs-List of <a href="/concepts/labels">labels</a>"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

A
dynamic_deps-List of <a href="/concepts/labels">labels</a>"[]
‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

M
;experimental_disable_topo_sort_do_not_use_remove_before_7_0Boolean"False
%
exports_filterList of strings"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

:
roots-List of <a href="/concepts/labels">labels</a>"[]

shared_lib_nameString"""
"
static_depsList of strings"[]
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

&
user_link_flagsList of strings"[]
ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

:
win_def_file$<a href="/concepts/labels">Label</a>"None(éä
java_import˚â

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

"
add_exportsList of strings"[]
 
	add_opensList of strings"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

"
constraintsList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

<
exports-List of <a href="/concepts/labels">labels</a>"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


7
jars-List of <a href="/concepts/labels">labels</a>(
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


	neverlinkBoolean"False
C
proguard_specs-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

A
runtime_deps-List of <a href="/concepts/labels">labels</a>"[]
4
srcjar$<a href="/concepts/labels">Label</a>"None
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(öé
java_libraryÜé

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

>
	resources-List of <a href="/concepts/labels">labels</a>"[]
"
add_exportsList of strings"[]
 
	add_opensList of strings"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

E
exported_plugins-List of <a href="/concepts/labels">labels</a>"[]
<
exports-List of <a href="/concepts/labels">labels</a>"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


 
	javacoptsList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


	neverlinkBoolean"False
<
plugins-List of <a href="/concepts/labels">labels</a>"[]
C
proguard_specs-List of <a href="/concepts/labels">labels</a>"[]
#
resource_strip_prefixString"""
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

A
runtime_deps-List of <a href="/concepts/labels">labels</a>"[]
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(ò{
java_lite_proto_library˙z

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(ﬁ}
java_proto_library≈}

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(–z
java_package_configurationØz

name(
˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


 
	javacoptsList of strings"[]
&
output_licensesList of strings"[]
=
packages-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(πç
java_plugin¶ç

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

>
	resources-List of <a href="/concepts/labels">labels</a>"[]
"
add_exportsList of strings"[]
 
	add_opensList of strings"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



generates_apiBoolean"False
 
	javacoptsList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


	neverlinkBoolean"False
&
output_licensesList of strings"[]
<
plugins-List of <a href="/concepts/labels">labels</a>"[]

processor_classString"""
C
proguard_specs-List of <a href="/concepts/labels">labels</a>"[]
#
resource_strip_prefixString"""
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(úw
java_runtimeâw

name(
è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

9
default_cds$<a href="/concepts/labels">Label</a>"None
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


B
hermetic_srcs-List of <a href="/concepts/labels">labels</a>"[]
I
hermetic_static_libs-List of <a href="/concepts/labels">labels</a>"[]
2
java$<a href="/concepts/labels">Label</a>"None

	java_homeString"""
8

lib_ct_sym$<a href="/concepts/labels">Label</a>"None
9
lib_modules$<a href="/concepts/labels">Label</a>"None
&
output_licensesList of strings"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>


versionInteger"0
ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(Ù{
j2objc_libraryﬂ{

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

$
entry_classesList of strings"[]
‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


=
jre_deps-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(°~
objc_importè~

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
;
archives-List of <a href="/concepts/labels">labels</a>(
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



includesList of strings"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

!

sdk_dylibsList of strings"[]
%
sdk_frameworksList of strings"[]
#
sdk_includesList of strings"[]
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

A
textual_hdrs-List of <a href="/concepts/labels">labels</a>"[]
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

*
weak_sdk_frameworksList of strings"[](°ç
objc_libraryçç

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>


coptsList of strings"[]

definesList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

 
enable_modulesBoolean"False
‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


H
implementation_deps-List of <a href="/concepts/labels">labels</a>"[]

includesList of strings"[]

linkoptsList of strings"[]
8

module_map$<a href="/concepts/labels">Label</a>"None

module_nameString"""
A
non_arc_srcs-List of <a href="/concepts/labels">labels</a>"[]
1
pch$<a href="/concepts/labels">Label</a>"None
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

!

sdk_dylibsList of strings"[]
%
sdk_frameworksList of strings"[]
#
sdk_includesList of strings"[]

stampBoolean"False
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

A
textual_hdrs-List of <a href="/concepts/labels">labels</a>"[]
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

*
weak_sdk_frameworksList of strings"[](¢ã
proto_libraryçã

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

;
allow_exports$<a href="/concepts/labels">Label</a>"None
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

<
exports-List of <a href="/concepts/labels">labels</a>"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



import_prefixString"""
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

"
strip_import_prefixString""/"
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(≈t
proto_lang_toolchain™t

name(
I
allowlist_different_package$<a href="/concepts/labels">Label</a>"None
G
blacklisted_protos-List of <a href="/concepts/labels">labels</a>"[]

command_lineString(
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



mnemonicString"
"GenProto"
 
output_filesString""legacy"
4
plugin$<a href="/concepts/labels">Label</a>"None
 
plugin_format_flagString"""
?
progress_messageString"#"Generating proto_library %{label}"
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

5
runtime$<a href="/concepts/labels">Label</a>"None
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

<
toolchain_type$<a href="/concepts/labels">Label</a>"None
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(©A built-in module to support native rules and other package helper functions. All native rules appear as functions in this module, e.g. <code>native.cc_library</code>. Note that the native module is only available in the loading phase (i.e. for macros, not for rule implementations). Attributes will ignore <code>None</code> values, and treat them as if the attribute was unset.<br>The following functions are also available:
Ô	
platform_common∏
ConstraintSettingInfoProvider"îThe constructor/key for the <a href='../providers/ConstraintSettingInfo.html'>ConstraintSettingInfo</a> provider.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>≤
ConstraintValueInfoProvider"êThe constructor/key for the <a href='../providers/ConstraintValueInfo.html'>ConstraintValueInfo</a> provider.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>ù
PlatformInfoProvider"ÇThe constructor/key for the <a href='../providers/PlatformInfo.html'>PlatformInfo</a> provider.<br/><i>Note: This API is experimental and may change at any time. It is disabled by default, but may be enabled with <code>--experimental_platforms_api</code></i>ë
TemplateVariableInfoProvider"oThe constructor/key for the <a href='../providers/TemplateVariableInfo.html'>TemplateVariableInfo</a> provider.|
ToolchainInfoProvider"aThe constructor/key for the <a href='../providers/ToolchainInfo.html'>ToolchainInfo</a> provider.:Functions for Starlark to interact with the platform APIs.
Õ
protoò
encode_text

x(string"˜Returns the struct argument's encoding as a text-format protocol message.
The data structure must be recursively composed of strings, ints, floats, or bools, or structs, sequences, and dicts of these types.
<p>A struct is converted to a message. Fields are emitted in name order.
Each struct field whose value is None is ignored.
<p>A sequence (such as a list or tuple) is converted to a repeated field.
Its elements must not be sequences or dicts.
<p>A dict is converted to a repeated field of messages with fields named 'key' and 'value'.
Entries are emitted in iteration (insertion) order.
The dict's keys must be strings or ints, and its values must not be sequences or dicts.
Examples:<br><pre class=language-python>proto.encode_text(struct(field=123))
# field: 123

proto.encode_text(struct(field=True))
# field: true

proto.encode_text(struct(field=[1, 2, 3]))
# field: 1
# field: 2
# field: 3

proto.encode_text(struct(field='text', ignored_field=None))
# field: "text"

proto.encode_text(struct(field=struct(inner_field='text', ignored_field=None)))
# field {
#   inner_field: "text"
# }

proto.encode_text(struct(field=[struct(inner_field=1), struct(inner_field=2)]))
# field {
#   inner_field: 1
# }
# field {
#   inner_field: 2
# }

proto.encode_text(struct(field=struct(inner_field=struct(inner_inner_field='text'))))
# field {
#    inner_field {
#     inner_inner_field: "text"
#   }
# }

proto.encode_text(struct(foo={4: 3, 2: 1}))
# foo: {
#   key: 4
#   value: 3
# }
# foo: {
#   key: 2
#   value: 1
# }
</pre>)A module for protocol message processing.
Ì
proto_common‹Utilities for protocol buffers. <p>Please consider using <code>load("@rules_proto//proto:defs.bzl", "proto_common")</code> to load this symbol from <a href="https://github.com/bazelbuild/rules_proto">rules_proto</a>.</p>
î
testing~
ExecutionInfoExecutionInfo"\<a href='../providers/ExecutionInfo.html'>testing.ExecutionInfo</a> provider key/constructor˘
TestEnvironment¢
†
environmentéA map of string keys and values that represent environment variables and their values. These will be made available during the test execution.(
Ë
inherited_environments<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s’A sequence of names of environment variables. These variables are made available during the test execution with their current value taken from the shell environment. If a variable is contained in both <code>environment</code> and <code>inherited_environment</code>, the value inherited from the shell environment will take precedence if set."[]RunEnvironmentInfo"¿<b>Deprecated: Use RunEnvironmentInfo instead.</b> Creates a new test environment provider. Use this provider to specify extra environment variables to be made available during test execution.Õ
analysis_testá
l
namebName of the target. It should be a Starlark identifier, matching pattern '[A-Za-z_][A-Za-z0-9_]*'.(
Ò
implementation‹The Starlark function implementing this analysis test. It must have exactly one parameter: <a href="../builtins/ctx.html">ctx</a>. The function is called during the analysis phase. It can access the attributes declared by <code>attrs</code> and populated via <code>attr_values</code>. The implementation function may not register actions. Instead, it must register a pass/fail result via providing <a href='../providers/AnalysisTestResultInfo.html'>AnalysisTestResultInfo</a>.(
Ω
attrs3<a class="anchor" href="../core/dict.html">dict</a>˙Dictionary declaring the attributes. See the <a href="../globals/bzl.html#rule">rule</a> call. Attributes are allowed to use configuration transitions defined using <a  href="../globals/bzl.html#analysis_test_transition">analysis_test_transition</a>."{}
‰
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s^List of configuration fragments that are available to the implementation of the analysis test."[]
≠

toolchains7<a class="anchor" href="../core/list.html">sequence</a>bThe set of toolchains the test requires. See the <a href="../globals/bzl.html#rule">rule</a> call."[]
¡
attr_valueso<a class="anchor" href="../core/dict.html">dict</a> of <a class="anchor" href="../core/string.html">string</a>s=Dictionary of attribute values to pass to the implementation."{}NoneType"±Creates a new analysis test target. <p>The number of transitive dependencies of the test are limited. The limit is controlled by <code>--analysis_testing_deps_limit</code> flag.=Helper methods for Starlark to access testing infrastructure.
¬
boolπA type to represent booleans. There are only two possible values: True and False. Any value can be converted to a boolean using the <a href="../globals/all.html#bool">bool</a> function.
T
builtin_function_or_method6The type of a built-in function, defined by Java code.
ı%
dict:
clear
NoneType"%Remove all items from the dictionary.Ê
get~

keyThe key to look for.(
T
defaultCThe default value to use (instead of None) if the key is not found."Noneunknown"ﬁReturns the value for <code>key</code> if <code>key</code> is in the dictionary, else <code>default</code>. If <code>default</code> is not given, it defaults to <code>None</code>, so that this method never throws an error.ù
itemslist"ãReturns the list of key-value tuples:<pre class="language-python">{2: "a", 4: "b", 1: "c"}.items() == [(2, "a"), (4, "b"), (1, "c")]</pre>
y
keyslist"iReturns the list of keys:<pre class="language-python">{2: "a", 4: "b", 1: "c"}.keys() == [2, 4, 1]</pre>
∆
popW

keyThe key.(
9
default%a default value if the key is absent."unboundunknown"ÂRemoves a <code>key</code> from the dict, and returns the associated value. If no entry with that key was found, remove nothing and return the specified <code>default</code> value; if no default value was specified, fail instead.ë
popitemtuple"¸Remove and return the first <code>(key, value)</code> pair from the dictionary. <code>popitem</code> is useful to destructively iterate over a dictionary, as often used in set algorithms. If the dictionary is empty, the <code>popitem</code> call fails.≠

setdefaultT

keyThe key.(
6
default%a default value if the key is absent."Noneunknown"»If <code>key</code> is in the dictionary, return its value. If not, insert key with a value of <code>default</code> and return <code>default</code>. <code>default</code> defaults to <code>None</code>.‡
updateº
}
pairspEither a dictionary or a list of entries. Entries must be tuples or lists with exactly two elements: key, value."[]
1
**kwargs!Dictionary of additional entries.(8NoneType"ñUpdates the dictionary first with the optional positional argument, <code>pairs</code>,  then with the optional keyword arguments
If the positional argument is present, it must be a dict, iterable, or None.
If it is a dict, then its key/value pairs are inserted into this dict. If it is an iterable, it must provide a sequence of pairs (or other iterables of length 2), each of which is treated as a key/value pair to be inserted.
Each keyword argument <code>name=value</code> causes the name/value pair to be inserted into this dict.Ö
valueslist"sReturns the list of values:<pre class="language-python">{2: "a", 4: "b", 1: "c"}.values() == ["a", "b", "c"]</pre>
¥dict is a built-in type representing an associative mapping or <i>dictionary</i>. A dictionary supports indexing using <code>d[k]</code> and key membership testing using <code>k in d</code>; both operations take constant time. Unfrozen dictionaries are mutable, and may be updated by assigning to <code>d[k]</code> or by calling certain methods. Dictionaries are iterable; iteration yields the sequence of keys in insertion order. Iteration order is unaffected by updating the value associated with an existing key, but is affected by removing then reinserting a key.
<pre>d = {0: 0, 2: 2, 1: 1}
[k for k in d]  # [0, 2, 1]
d.pop(2)
d[0], d[2] = "a", "b"
0 in d, "a" in d  # (True, False)
[(k, v) for k, v in d.items()]  # [(0, "a"), (1, 1), (2, "b")]
</pre>
<p>There are four ways to construct a dictionary:
<ol>
<li>A dictionary expression <code>{k: v, ...}</code> yields a new dictionary with the specified key/value entries, inserted in the order they appear in the expression. Evaluation fails if any two key expressions yield the same value.
<li>A dictionary comprehension <code>{k: v for vars in seq}</code> yields a new dictionary into which each key/value pair is inserted in loop iteration order. Duplicates are permitted: the first insertion of a given key determines its position in the sequence, and the last determines its associated value.
<pre class="language-python">
{k: v for k, v in (("a", 0), ("b", 1), ("a", 2))}  # {"a": 2, "b": 1}
{i: 2*i for i in range(3)}  # {0: 0, 1: 2, 2: 4}
</pre>
<li>A call to the built-in <a href="../globals/all.html#dict">dict</a> function returns a dictionary containing the specified entries, which are inserted in argument order, positional arguments before named. As with comprehensions, duplicate keys are permitted.
<li>The union expression <code>x | y</code> yields a new dictionary by combining two existing dictionaries. If the two dictionaries have a key <code>k</code> in common, the right hand side dictionary's value of the key (in other words, <code>y[k]</code>) wins. The <code>|=</code> variant of the union operator modifies a dictionary in-place. Example:<br><pre class=language-python>d = {"foo": "FOO", "bar": "BAR"} | {"foo": "FOO2", "baz": "BAZ"}
# d == {"foo": "FOO2", "bar": "BAR", "baz": "BAZ"}
d = {"a": 1, "b": 2}
d |= {"b": 3, "c": 4}
# d == {"a": 1, "b": 3, "c": 4}</pre></ol>
8
float/The type of floating-point numbers in Starlark.
7
function+The type of functions declared in Starlark.
µ
int≠The type of integers in Starlark. Starlark integers may be of any magnitude; arithmetic is exact. Examples of integer expressions:<br><pre class="language-python">153
0x2A  # hexadecimal literal
0o54  # octal literal
23 * 2 + 5
100 / -7
100 % -7  # -5 (unlike in some other languages)
int("18")
</pre>
Û
json£	
decodeÜ

xJSON string to decode.(
\
defaultHIf specified, the value to return when <code>x</code> cannot be decoded."unboundunknown"èThe decode function has one required positional parameter: a JSON string.
It returns the Starlark value that the string denotes.
<ul><li><code>"null"</code>, <code>"true"</code> and <code>"false"</code> are parsed as <code>None</code>, <code>True</code>, and <code>False</code>.
<li>Numbers are parsed as int, or as a float if they contain a decimal point or an exponent. Although JSON has no syntax  for non-finite values, very large values may be decoded as infinity.
<li>a JSON object is parsed as a new unfrozen Starlark dict. If the same key string occurs more than once in the object, the last value for the key is kept.
<li>a JSON array is parsed as new unfrozen Starlark list.
</ul>
If <code>x</code> is not a valid JSON encoding and the optional <code>default</code> parameter is specified (including specified as <code>None</code>), this function returns the <code>default</code> value.
If <code>x</code> is not a valid JSON encoding and the optional <code>default</code> parameter is <em>not</em> specified, this function fails.œ
encode

x(string"≥<p>The encode function accepts one required positional argument, which it converts to JSON by cases:
<ul>
<li>None, True, and False are converted to 'null', 'true', and 'false', respectively.
<li>An int, no matter how large, is encoded as a decimal integer. Some decoders may not be able to decode very large integers.
<li>A float is encoded using a decimal point or an exponent or both, even if its numeric value is an integer. It is an error to encode a non-finite  floating-point value.
<li>A string value is encoded as a JSON string literal that denotes the value.  Each unpaired surrogate is replaced by U+FFFD.
<li>A dict is encoded as a JSON object, in key order.  It is an error if any key is not a string.
<li>A list or tuple is encoded as a JSON array.
<li>A struct-like value is encoded as a JSON object, in field name order.
</ul>
An application-defined type may define its own JSON encoding.
Encoding any other value yields an error.
›
encode_indent-

x(

prefix"''

indent"'\t'string"úThe encode_indent function is equivalent to <code>json.indent(json.encode(x), ...)</code>. See <code>indent</code> for description of formatting parameters.’
indent-

s(

prefix"''

indent"'\t'string"õThe indent function returns the indented form of a valid JSON-encoded string.
Each array element or object field appears on a new line, beginning with the prefix string followed by one or more copies of the indent string, according to its nesting depth.
The function accepts one required positional parameter, the JSON string,
and two optional keyword-only string parameters, prefix and indent,
that specify a prefix of each new line, and the unit of indentation.
If the input is not valid, the function may fail or return invalid output.
;Module json is a Starlark module of JSON-related functions.
ì
list]
append-
!
itemItem to add at the end.(NoneType"$Adds an item to the end of the list.:
clear
NoneType"%Removes all the elements of the list.a
extend/
#
itemsItems to add at the end.(NoneType"&Adds all items to the end of the list.©
index≥

xThe object to search.(
á
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>/The start index of the list portion to inspect."None
É
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>-The end index of the list portion to inspect."Noneint"jReturns the index in the list of the first item whose value is x. It is an error if there is no such item.|
insertL
+
index The index of the given position.(

item	The item.(NoneType"$Inserts an item at a given position.ô
pops
h
iG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>The index of the item."-1unknown"úRemoves the item at the given position in the list, and returns it. If no <code>index</code> is specified, it removes and returns the last item in the list.ì
remove(

xThe object to remove.(NoneType"_Removes the first item from the list whose value is x. It is an error if there is no such item.∞The built-in list type. Example list expressions:<br><pre class=language-python>x = [1, 2, 3]</pre>Accessing elements is possible using indexing (starts from <code>0</code>):<br><pre class=language-python>e = x[1]   # e == 2</pre>Lists support the <code>+</code> operator to concatenate two lists. Example:<br><pre class=language-python>x = [1, 2] + [3, 4]   # x == [1, 2, 3, 4]
x = ["a", "b"]
x += ["c"]            # x == ["a", "b", "c"]</pre>Similar to strings, lists support slice operations:<pre class=language-python>['a', 'b', 'c', 'd'][1:3]   # ['b', 'c']
['a', 'b', 'c', 'd'][::2]  # ['a', 'c']
['a', 'b', 'c', 'd'][3:0:-1]  # ['d', 'c', 'b']</pre>Lists are mutable, as in Python.
≠
range£A language built-in type to support ranges. Example of range literal:<br><pre class=language-python>x = range(1, 10, 3)</pre>Accessing elements is possible using indexing (starts from <code>0</code>):<br><pre class=language-python>e = x[1]   # e == 2</pre>Ranges do not support the <code>+</code> operator for concatenation.Similar to strings, ranges support slice operations:<pre class=language-python>range(10)[1:3]   # range(1, 3)
range(10)[::2]  # range(0, 10, 2)
range(10)[3:0:-1]  # range(3, 0, -1)</pre>Ranges are immutable, as in Python 3.
ÌZ
string∞

capitalizestring"óReturns a copy of the string with its first character (if any) capitalized and the rest lowercased. This method does not support non-ascii characters. ò
count≤
 
subThe substring to count.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"ŸReturns the number of (non-overlapping) occurrences of substring <code>sub</code> in string, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.◊
elems
sequence"¡Returns an iterable value containing successive 1-element substrings of the string. Equivalent to <code>[s[i] for i in range(len(s))]</code>, except that the returned value might not be a list. 
endswithˆ

sub≠<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/tuple.html">tuple</a> of <a class="anchor" href="../core/string.html">string</a>s7The suffix (or tuple of alternative suffixes) to match.(
u
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code> Test beginning at this position."0
É
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>-optional position at which to stop comparing."Nonebool"ƒReturns True if the string ends with <code>sub</code>, otherwise False, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.ë
find±

subThe substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"‘Returns the first index where <code>sub</code> is found, or -1 if no such index exists, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.◊
formatW
!
*argsList of arguments."()0
*
**kwargsDictionary of arguments."{}8string"ÛPerform string interpolation. Format strings contain replacement fields surrounded by curly braces <code>&#123;&#125;</code>. Anything that is not contained in braces is considered literal text, which is copied unchanged to the output.If you need to include a brace character in the literal text, it can be escaped by doubling: <code>&#123;&#123;</code> and <code>&#125;&#125;</code>A replacement field can be either a name, a number, or empty. Values are converted to strings using the <a href="../globals/all.html#str">str</a> function.<pre class="language-python"># Access in order:
"&#123;&#125; < &#123;&#125;".format(4, 5) == "4 < 5"
# Access by position:
"{1}, {0}".format(2, 1) == "1, 2"
# Access by name:
"x{key}x".format(key = 2) == "x2x"</pre>
û
index±

subThe substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"‡Returns the first index where <code>sub</code> is found, or raises an error if no such  index exists, optionally restricting to <code>[start:end]</code><code>start</code> being inclusive and <code>end</code> being exclusive.É
isalnumbool"pReturns True if all characters in the string are alphanumeric ([a-zA-Z0-9]) and there is at least one character.~
isalphabool"kReturns True if all characters in the string are alphabetic ([a-zA-Z]) and there is at least one character.w
isdigitbool"dReturns True if all characters in the string are digits ([0-9]) and there is at least one character.x
islowerbool"eReturns True if all cased characters in the string are lowercase and there is at least one character.|
isspacebool"iReturns True if all characters are white space characters and the string contains at least one character.ì
istitlebool"ˇReturns True if the string is in title case and it contains at least one character. This means that every uppercase character must follow an uncased one (e.g. whitespace) and every lowercase character must follow a cased one (e.g. uppercase or lowercase).x
isupperbool"eReturns True if all cased characters in the string are uppercase and there is at least one character.¯
join,
"
elementsThe objects to join.(string"¡Returns a string in which the string elements of the argument have been joined by this string as a separator. Example:<br><pre class="language-python">"|".join(["a", "b", "c"]) == "a|b|c"</pre>A
lowerstring".Returns the lower case version of this string.®
lstripù
í
charsM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>4The characters to remove, or all whitespace if None."Nonestring"˝Returns a copy of the string where leading characters that appear in <code>chars</code> are removed. Note that <code>chars</code> is not a prefix: all combinations of its value are removed:<pre class="language-python">"abcba".lstrip("ba") == "cba"</pre>≈
	partition)
 
sepThe string to split on.(tuple"åSplits the input string at the first occurrence of the separator <code>sep</code> and returns the resulting partition as a three-element tuple of the form (before, separator, after). If the input string does not contain the separator, partition returns (self, '', '').√
removeprefix6
,
prefix The prefix to remove if present.(string"{If the string starts with <code>prefix</code>, returns a new string with the prefix removed. Otherwise, returns the string.¡
removesuffix6
,
suffix The suffix to remove if present.(string"yIf the string ends with <code>suffix</code>, returns a new string with the suffix removed. Otherwise, returns the string.å
replace¡
#
oldThe string to be replaced.(
$
newThe string to replace with.(
l
count_The maximum number of replacements. If omitted, or if the value is negative, there is no limit."-1string"ºReturns a copy of the string in which the occurrences of <code>old</code> have been replaced with <code>new</code>, optionally restricting the number of replacements to <code>count</code>.ë
rfind±

subThe substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"”Returns the last index where <code>sub</code> is found, or -1 if no such index exists, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.ü
rindex±

subThe substring to find.(
{
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>&Restrict to search from this position."0
ã
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>5optional position before which to restrict to search."Noneint"‡Returns the last index where <code>sub</code> is found, or raises an error if no such index exists, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.∆

rpartition)
 
sepThe string to split on.(tuple"åSplits the input string at the last occurrence of the separator <code>sep</code> and returns the resulting partition as a three-element tuple of the form (before, separator, after). If the input string does not contain the separator, rpartition returns ('', '', self).ë
rsplit¢
 
sepThe string to split on.(
x
maxsplitG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>The maximum number of splits."Nonelist"·Returns a list of all the words in the string, using <code>sep</code> as the separator, optionally limiting the number of splits to <code>maxsplit</code>. Except for splitting from the right, this method behaves like split().™
rstripù
í
charsM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>4The characters to remove, or all whitespace if None."Nonestring"ˇReturns a copy of the string where trailing characters that appear in <code>chars</code> are removed. Note that <code>chars</code> is not a suffix: all combinations of its value are removed:<pre class="language-python">"abcbaa".rstrip("ab") == "abc"</pre>…
split¢
 
sepThe string to split on.(
x
maxsplitG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>The maximum number of splits."Nonelist"öReturns a list of all the words in the string, using <code>sep</code> as the separator, optionally limiting the number of splits to <code>maxsplit</code>.◊

splitlines`
T
keependsAWhether the line breaks should be included in the resulting list."Falsesequence"gSplits the string at line boundaries ('\n', '\r\n', '\r') and returns the result as a new mutable list.¿

startswithË

sub≠<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../core/tuple.html">tuple</a> of <a class="anchor" href="../core/string.html">string</a>s7The prefix (or tuple of alternative prefixes) to match.(
u
startG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code> Test beginning at this position."0
v
endG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code> Stop comparing at this position."Nonebool"∆Returns True if the string starts with <code>sub</code>, otherwise False, optionally restricting to <code>[start:end]</code>, <code>start</code> being inclusive and <code>end</code> being exclusive.«
stripù
í
charsM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>4The characters to remove, or all whitespace if None."Nonestring"ùReturns a copy of the string where leading or trailing characters that appear in <code>chars</code> are removed. Note that <code>chars</code> is neither a prefix nor a suffix: all combinations of its value are removed:<pre class="language-python">"aabcbcbaa".strip("ab") == "cbc"</pre>ñ
titlestring"ÇConverts the input string into title case, i.e. every word starts with an uppercase letter while the remaining letters are lowercase. In this context, a word means strictly a sequence of letters. This method does not support supplementary Unicode characters.A
upperstring".Returns the upper case version of this string.æA language built-in type to support strings. Examples of string literals:<br><pre class="language-python">a = 'abc\ndef'
b = "ab'cd"
c = """multiline string"""

# Strings support slicing (negative index starts from the end):
x = "hello"[2:4]  # "ll"
y = "hello"[1:-1]  # "ell"
z = "hello"[:4]  # "hell"# Slice steps can be used, too:
s = "hello"[::2] # "hlo"
t = "hello"[3:0:-1] # "lle"
</pre>Strings are not directly iterable, use the <code>.elems()</code> method to iterate over their characters. Examples:<br><pre class="language-python">"bc" in "abcd"   # evaluates to True
x = [c for c in "abc".elems()]  # x == ["a", "b", "c"]</pre>
Implicit concatenation of strings is not allowed; use the <code>+</code> operator instead. Comparison operators perform a lexicographical comparison; use <code>==</code> to test for equality.
Ê
tuple‹The built-in tuple type. Example tuple expressions:<br><pre class=language-python>x = (1, 2, 3)</pre>Accessing elements is possible using indexing (starts from <code>0</code>):<br><pre class=language-python>e = x[1]   # e == 2</pre>Lists support the <code>+</code> operator to concatenate two tuples. Example:<br><pre class=language-python>x = (1, 2) + (3, 4)   # x == (1, 2, 3, 4)
x = ("a", "b")
x += ("c",)            # x == ("a", "b", "c")</pre>Similar to lists, tuples support slice operations:<pre class=language-python>('a', 'b', 'c', 'd')[1:3]   # ('b', 'c')
('a', 'b', 'c', 'd')[::2]  # ('a', 'c')
('a', 'b', 'c', 'd')[3:0:-1]  # ('d', 'c', 'b')</pre>Tuples are immutable, therefore <code>x[1] = "a"</code> is not supported.ë
existing_rule,
!
nameThe name of the target.(unknown"œReturns an immutable dict-like object that describes the attributes of a rule instantiated in this thread's package, or <code>None</code> if no rule instance of that name exists.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>If the <code>--noincompatible_existing_rules_immutable_view</code> flag is set, instead returns a new mutable dict with the same content.<p>The result contains an entry for each attribute, with the exception of private ones (whose names do not start with a letter) and a few unrepresentable legacy attribute types. In addition, the dict contains entries for the rule instance's <code>name</code> and <code>kind</code> (for example, <code>'cc_binary'</code>).<p>The values of the result represent attribute values as follows:<ul><li>Attributes of type str, int, and bool are represented as is.</li><li>Labels are converted to strings of the form <code>':foo'</code> for targets in the same package or <code>'//pkg:name'</code> for targets in a different package.</li><li>Lists are represented as tuples, and dicts are converted to new, mutable dicts. Their elements are recursively converted in the same fashion.</li><li><code>select</code> values are returned with their contents transformed as described above.</li><li>Attributes for which no value was specified during rule instantiation and whose default value is computed are excluded from the result. (Computed defaults cannot be computed until the analysis phase.).</li></ul><p>If possible, avoid using this function. It makes BUILD files brittle and order-dependent. Also, beware that it differs subtly from the two other conversions of rule attribute values from internal form to Starlark: one used by computed defaults, the other used by <code>ctx.attr.foo</code>.(Â
existing_rules	unknown"≈Returns an immutable dict-like object describing the rules so far instantiated in this thread's package. Each entry of the dict-like object maps the name of the rule instance to the result that would be returned by <code>existing_rule(name)</code>.<p>Here, an <em>immutable dict-like object</em> means a deeply immutable object <code>x</code> supporting dict-like iteration, <code>len(x)</code>, <code>name in x</code>, <code>x[name]</code>, <code>x.get(name)</code>, <code>x.items()</code>, <code>x.keys()</code>, and <code>x.values()</code>.<p>If the <code>--noincompatible_existing_rules_immutable_view</code> flag is set, instead returns a new mutable dict with the same content.<p><em>Note: If possible, avoid using this function. It makes BUILD files brittle and order-dependent. Furthermore, if the </em><code>--noincompatible_existing_rules_immutable_view</code><em> flag is set, this function may be very expensive, especially if called within a loop.</em>(„
exports_filesı
õ
srcss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sThe list of files to export.(
ê

visibilityM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>¨A visibility declaration can to be specified. The files will be visible to the targets specified. If no visibility is specified, the files will be visible to every package."None
∑
licensesâ<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <code>None</code>Licenses to be specified."NoneNoneType"XSpecifies a list of files belonging to this package that are exported to other packages.(ﬂ
globØ
©
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to include."[]
©
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s%The list of glob patterns to exclude."[]
G
exclude_directories-A flag whether to exclude directories or not."1
Å
allow_emptyËWhether we allow glob patterns to match nothing. If `allow_empty` is False, each individual include pattern must match something and also the final result must be non-empty (after the matches of the `exclude` patterns are excluded)."unboundsequence"¢Glob returns a new, mutable, sorted list of every file in the current package that:<ul>
<li>Matches at least one pattern in <code>include</code>.</li>
<li>Does not match any of the patterns in <code>exclude</code> (default <code>[]</code>).</li></ul>
If the <code>exclude_directories</code> argument is enabled (set to <code>1</code>), files of type directory will be omitted from the results (default <code>1</code>).(˛
module_namestring"‚The name of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the name of the module hosting the extension. It's the same as the <code>module.name</code> field seen in <code>module_ctx.modules</code>.(ä
module_versionstring"ÎThe version of the Bazel module associated with the repo this package is in. If this package is from a repo defined in WORKSPACE instead of MODULE.bazel, this is empty. For repos generated by module extensions, this is the version of the module hosting the extension. It's the same as the <code>module.version</code> field seen in <code>module_ctx.modules</code>.(À
package_group®
(
nameThe unique name for this rule.(
∂
packagess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s1A complete enumeration of packages in this group."[]
∏
includess<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s3Other package groups that are included in this one."[]NoneType"åThis function defines a set of packages and assigns a label to the group. The label can be referenced in <code>visibility</code> attributes.(–
package_namestring"≥The name of the package being evaluated, without the repository name. For example, in the BUILD file <code>some/package/BUILD</code>, its value will be <code>some/package</code>. If the BUILD file calls a function defined in a .bzl file, <code>package_name()</code> will match the caller BUILD file package.(π
package_relative_labelÂ
€
inputu<a class="anchor" href="../core/string.html">string</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>YThe input label string or Label object. If a Label object is passed, it's returned as is.(Label"¥Converts the input string into a <a href='../builtins/Label.html'>Label</a> object, in the context of the package currently being initialized (that is, the <code>BUILD</code> file for which the current macro is executing). If the input is already a <code>Label</code>, it is returned unchanged.<p>This function may only be called while evaluating a BUILD file and the macros it directly or indirectly calls; it may not be called in (for instance) a rule implementation function. <p>The result of this function is the same <code>Label</code> value as would be produced by passing the given string to a label-valued attribute of a target declared in the BUILD file. <p><i>Usage note:</i> The difference between this function and <a href='../builtins/Label.html#Label'>Label()</a></code> is that <code>Label()</code> uses the context of the package of the <code>.bzl</code> file that called it, not the package of the <code>BUILD</code> file. Use <code>Label()</code> when you need to refer to a fixed target that is hardcoded into the macro, such as a compiler. Use <code>package_relative_label()</code> when you need to normalize a label string supplied by the BUILD file to a <code>Label</code> object. (There is no way to convert a string to a <code>Label</code> in the context of a package other than the BUILD file or the calling .bzl file. For that reason, outer macros should always prefer to pass Label objects to inner macros rather than label strings.)(â
	repo_namestring"pThe canonical name of the repository containing the package currently being evaluated, with no leading at-signs.(±
repository_namestring"ë<b>Experimental</b>. This API is experimental and may change at any time. Please do not depend on it. It may be enabled on an experimental basis by setting <code>--+incompatible_enable_deprecated_label_apis</code> <br><strong>Deprecated.</strong> Prefer to use <a href="#repo_name"><code>repo_name</code></a> instead, which doesn't contain the spurious leading at-sign, but behaves identically otherwise.<p>The canonical name of the repository containing the package currently being evaluated, with a single at-sign (<code>@</code>) prefixed. For example, in packages that are called into existence by the WORKSPACE stanza <code>local_repository(name='local', path=...)</code> it will be set to <code>@local</code>. In packages in the main repository, it will be set to <code>@</code>.(Á
subpackagesÇ
ª
includes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s9The list of glob patterns to include in subpackages scan.(
ø
excludes<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s;The list of glob patterns to exclude from subpackages scan."[]
ı
allow_emptyﬁWhether we fail if the call returns an empty list. By default empty list indicates potential error in BUILD file where the call to subpackages() is superflous.  Setting to true allows this function to succeed in that case."Falsesequence"–Returns a new mutable list of every direct subpackage of the current package, regardless of file-system directory depth. List returned is sorted and contains the names of subpackages relative to the current package. It is advised to prefer using the methods in bazel_skylib.subpackages module rather than calling this function directly.(
Falsebool
Truebool
NoneNoneType¨
absó
ã
xk<a class="anchor" href="../core/int.html">int</a>; or <a class="anchor" href="../core/float.html">float</a>A number (int or float)(unknown"äReturns the absolute value of a number (a non-negative number with the same magnitude).<pre class="language-python">abs(-2.3) == 2.3</pre>ª
all;
3
elements%A string or a collection of elements.(bool"ˆReturns true if all elements evaluate to True or if the collection is empty. Elements are converted to boolean using the <a href="#bool">bool</a> function.<pre class="language-python">all(["hello", 3, True]) == True
all([-1, 0, 1]) == False</pre>¢
any;
3
elements%A string or a collection of elements.(bool"›Returns true if at least one element evaluates to True. Elements are converted to boolean using the <a href="#bool">bool</a> function.<pre class="language-python">any([-1, 0, 1]) == True
any([False, 0, ""]) == False</pre>“
bool,
$
xThe variable to convert."Falsebool"õConstructor for the bool type. It returns <code>False</code> if the object is <code>None</code>, <code>False</code>, an empty string (<code>""</code>), the number <code>0</code>, or an empty collection (e.g. <code>()</code>, <code>[]</code>). Otherwise, it returns <code>True</code>.Ê
dictê
U
pairsHA dict, or an iterable whose elements are each of length 2 (key, value)."[]
1
**kwargs!Dictionary of additional entries.(8dict" Creates a <a href="../core/dict.html">dictionary</a> from an optional positional argument and an optional set of keyword arguments. In the case where the same key is given multiple times, the last value will be used. Entries supplied via keyword arguments are considered to come after entries supplied via the positional argument.á
dir#

xThe object to check.(list"[Returns a list of strings: the names of the attributes and methods of the parameter object.é
	enumerate;

listinput sequence.(

startstart index."0list"√Returns a list of pairs (two-element tuples), with the index (int) and the item from the input sequence.
<pre class="language-python">enumerate([24, 21, 84]) == [(0, 24), (1, 21), (2, 84)]</pre>
ï
fail„
{
msgnDeprecated: use positional arguments instead. This argument acts like an implicit leading positional argument."None
ª
attrM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>^Deprecated. Causes an optional prefix containing this string to be added to the error message."None
õ
*argsçA list of values, formatted with debugPrint (which is equivalent to str by default) and joined with spaces, that appear in the error message.(0NoneType"'Causes execution to fail with an error.‡
float,
#
xThe value to convert."unboundfloat"®Returns x as a float value. <ul><li>If <code>x</code> is already a float, <code>float</code> returns it unchanged. <li>If <code>x</code> is a bool, <code>float</code> returns 1.0 for True and 0.0 for False. <li>If <code>x</code> is an int, <code>float</code> returns the nearest finite floating-point value to x, or an error if the magnitude is too large. <li>If <code>x</code> is a string, it must be a valid floating-point literal, or be equal (ignoring case) to <code>NaN</code>, <code>Inf</code>, or <code>Infinity</code>, optionally preceded by a <code>+</code> or <code>-</code> sign. </ul>Any other value causes an error. With no argument, <code>float()</code> returns 0.0.£
getattr◊
.
x'The struct whose attribute is accessed.(
+
name!The name of the struct attribute.(
o
default[The default value to return in case the struct doesn't have an attribute of the given name."unboundunknown"ΩReturns the struct's field of the given name if it exists. If not, it either returns <code>default</code> (if specified) or raises an error. <code>getattr(x, "foobar")</code> is equivalent to <code>x.foobar</code>.<pre class="language-python">getattr(ctx.attr, "myattr")
getattr(ctx.attr, "myattr", "mydefault")</pre>ó
hasattrI

xThe object to check.(
$
nameThe name of the attribute.(bool"¿Returns True if the object <code>x</code> has an attribute or method of the given <code>name</code>, otherwise False. Example:<br><pre class="language-python">hasattr(ctx.attr, "myattr")</pre>◊
hash'
 
valueString value to hash.(int"•Return a hash value for a string. This is computed deterministically using the same algorithm as Java's <code>String.hashCode()</code>, namely: <pre class="language-python">s[0] * (31^(n-1)) + s[1] * (31^(n-2)) + ... + s[n-1]</pre> Hashing of values besides strings is not currently supported.§
int°

xThe string to convert.(
˙
baseËThe base used to interpret a string value; defaults to 10. Must be between 2 and 36 (inclusive), or 0 to detect the base as if <code>x</code> were an integer literal. This parameter must not be supplied if the value is not a string."unboundint"¯Returns x as an int value.<ul><li>If <code>x</code> is already an int, <code>int</code> returns it unchanged.<li>If <code>x</code> is a bool, <code>int</code> returns 1 for True and 0 for False.<li>If <code>x</code> is a string, it must have the format     <code>&lt;sign&gt;&lt;prefix&gt;&lt;digits&gt;</code>.     <code>&lt;sign&gt;</code> is either <code>"+"</code>, <code>"-"</code>,     or empty (interpreted as positive). <code>&lt;digits&gt;</code> are a     sequence of digits from 0 up to <code>base</code> - 1, where the letters a-z     (or equivalently, A-Z) are used as digits for 10-35. In the case where     <code>base</code> is 2/8/16, <code>&lt;prefix&gt;</code> is optional and may     be 0b/0o/0x (or equivalently, 0B/0O/0X) respectively; if the     <code>base</code> is any other value besides these bases or the special value     0, the prefix must be empty. In the case where <code>base</code> is 0, the     string is interpreted as an integer literal, in the sense that one of the     bases 2/8/10/16 is chosen depending on which prefix if any is used. If     <code>base</code> is 0, no prefix is used, and there is more than one digit,     the leading digit cannot be 0; this is to avoid confusion between octal and     decimal. The magnitude of the number represented by the string must be within     the allowed range for the int type.<li>If <code>x</code> is a float, <code>int</code> returns the integer value of    the float, rounding towards zero. It is an error if x is non-finite (NaN or    infinity).</ul>This function fails if <code>x</code> is any other type, or if the value is a string not satisfying the above format. Unlike Python's <code>int</code> function, this function does not allow zero arguments, and does not allow extraneous whitespace for string arguments.<p>Examples:<pre class="language-python">int("123") == 123
int("-123") == -123
int("+123") == 123
int("FF", 16) == 255
int("0xFF", 16) == 255
int("10", 0) == 10
int("-0x10", 0) == -16
int("-0x10", 0) == -16
int("123.456") == 123
</pre>î
len/
(
x!The value whose length to report.(int"\Returns the length of a string, sequence (such as a list or tuple), dict, or other iterable.˙
list'

xThe object to convert."[]list"»Returns a new list with the same elements as the given iterable value.<pre class="language-python">list([1, 2]) == [1, 2]
list((2, 3, 2)) == [2, 3, 2]
list({5: "a", 2: "b", 4: "c"}) == [5, 2, 4]</pre>‚
max3
(
*argsThe elements to be checked.(0unknown"•Returns the largest one of all given arguments. If only one argument is provided, it must be a non-empty iterable.It is an error if elements are not comparable (for example int with string), or if no arguments are given. <pre class="language-python">max(2, 5, 4) == 5
max([5, 6, 3]) == 6</pre>‰
min3
(
*argsThe elements to be checked.(0unknown"ßReturns the smallest one of all given arguments. If only one argument is provided, it must be a non-empty iterable. It is an error if elements are not comparable (for example int with string), or if no arguments are given. <pre class="language-python">min(2, 5, 4) == 2
min([5, 6, 3]) == 3</pre>ˇ
print}
M
sepAThe separator string between the objects, default is space (" ")."" "
"
*argsThe objects to print.(0NoneType"ˆPrints <code>args</code> as debug output. It will be prefixed with the string <code>"DEBUG"</code> and the location (file and line number) of this call. The exact way in which the arguments are converted to strings is unspecified and may change at any time. In particular, it may be different from (and more detailed than) the formatting done by <a href='#str'><code>str()</code></a> and <a href='#repr'><code>repr()</code></a>.<p>Using <code>print</code> in production code is discouraged due to the spam it creates for users. For deprecations, prefer a hard error using <a href="#fail"><code>fail()</code></a> whenever possible.
range¥
t
start_or_stopaValue of the start element if stop is provided, otherwise value of stop and the actual start is 0(
Û
stop_or_noneG<a class="anchor" href="../core/int.html">int</a>; or <code>None</code>ìoptional index of the first item <i>not</i> to be included in the resulting list; generation of the list stops before <code>stop</code> is reached."None
<
step1The increment (default is 1). It may be negative."1sequence"ØCreates a list where items go from <code>start</code> to <code>stop</code>, using a <code>step</code> increment. If a single argument is provided, items will range from 0 to that element.<pre class="language-python">range(4) == [0, 1, 2, 3]
range(3, 9, 2) == [3, 5, 7]
range(3, 0, -1) == [3, 2, 1]</pre>∫
repr'

xThe object to convert.(string"àConverts any object to a string representation. This is useful for debugging.<br><pre class="language-python">repr("ab") == '"ab"'</pre>Ö
reversedG
?
sequence1The iterable sequence (e.g. list) to be reversed.(list"ØReturns a new, unfrozen list that contains the elements of the original iterable sequence in reversed order.<pre class="language-python">reversed([3, 5, 4]) == [4, 5, 3]</pre>Ç
sortedπ
,
iterableThe iterable sequence to sort.(
L
key?An optional function applied to each element before comparison."None
5
reverse#Return results in descending order."Falselist"ªReturns a new sorted list containing all the elements of the supplied iterable sequence. An error may occur if any pair of elements x, y may not be compared using x < y. The elements are sorted into ascending order, unless the reverse argument is True, in which case the order is descending.
 Sorting is stable: elements that compare equal retain their original relative order.
<pre class="language-python">sorted([3, 5, 4]) == [3, 4, 5]</pre>Æ
str'

xThe object to convert.(string"~Converts any object to string. This is useful for debugging.<pre class="language-python">str("ab") == "ab"
str(8) == "8"</pre>¸
tuple(

xThe object to convert."()tuple"»Returns a tuple with the same elements as the given iterable value.<pre class="language-python">tuple([1, 2]) == (1, 2)
tuple((2, 3, 2)) == (2, 3, 2)
tuple({5: "a", 2: "b", 4: "c"}) == (5, 2, 4)</pre>–
type-
#
xThe object to check type of.(string"òReturns the type name of its argument. This is useful for debugging and type-checking. Examples:<pre class="language-python">type(2) == "int"
type([1]) == "list"
type(struct(a = 2)) == "struct"</pre>This function might change in the future. To write Python-compatible code and be future-proof, use it only to compare return values: <pre class="language-python">if type(x) == type([]):  # if x is a list</pre>ç
zip"

*argslists to zip.(0list"·Returns a <code>list</code> of <code>tuple</code>s, where the i-th tuple contains the i-th element from each of the argument sequences or iterables. The list has the size of the shortest input. With a single iterable argument, it returns a list of 1-tuples. With no arguments, it returns an empty list. Examples:<pre class="language-python">zip()  # == []
zip([1, 2])  # == [(1,), (2,)]
zip([1, 2], [3, 4])  # == [(1, 3), (2, 4)]
zip([1, 2], [3, 4, 5])  # == [(1, 3), (2, 4)]</pre>æ
nativenative"©A built-in module to support native rules and other package helper functions. All native rules appear as functions in this module, e.g. <code>native.cc_library</code>. Note that the native module is only available in the loading phase (i.e. for macros, not for rule implementations). Attributes will ignore <code>None</code> values, and treat them as if the attribute was unset.<br>The following functions are also available:(ç
depsetï
ç
directM<a class="anchor" href="../core/list.html">sequence</a>; or <code>None</code>.A list of <i>direct</i> elements of a depset. "None
Ü
orderrThe traversal strategy for the new depset. See <a href="../builtins/depset.html">here</a> for the possible values."	"default"
Ò

transitiveç<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/depset.html">depset</a>s; or <code>None</code>MA list of depsets whose elements will become indirect elements of the depset."Nonedepset"Ë
Creates a <a href="../builtins/depset.html">depset</a>. The <code>direct</code> parameter is a list of direct elements of the depset, and <code>transitive</code> parameter is a list of depsets whose elements become indirect elements of the created depset. The order in which elements are returned when the depset is converted to a list is specified by the <code>order</code> parameter. See the <a href="https://bazel.build/extending/depsets">Depsets overview</a> for more information.
<p>All elements (direct and indirect) of a depset must be of the same type, as obtained by the expression <code>type(x)</code>.
<p>Because a hash-based set is used to eliminate duplicates during iteration, all elements of a depset should be hashable. However, this invariant is not currently checked consistently in all constructors. Use the --incompatible_always_check_depset_elements flag to enable consistent checking; this will be the default behavior in future releases;  see <a href='https://github.com/bazelbuild/bazel/issues/10313'>Issue 10313</a>.
<p>In addition, elements must currently be immutable, though this restriction will be relaxed in future.
<p> The order of the created depset should be <i>compatible</i> with the order of its <code>transitive</code> depsets. <code>"default"</code> order is compatible with any other order, all other orders are only compatible with themselves.(K
jsonjson";Module json is a Starlark module of JSON-related functions.(;
protoproto")A module for protocol message processing.(ç
selectØ
”
xÀA dict that maps configuration conditions to values. Each key is a <a href="../builtins/Label.html">Label</a> or a label string that identifies a config_setting or constraint_value instance. See the <a href="https://bazel.build/rules/macros#label-resolution">documentation on macros</a> for when to use a Label instead of a string.(
N
no_match_error8Optional custom error to report if no condition matches."''unknown"Œ<code>select()</code> is the helper function that makes a rule attribute <a href="#configurable-attributes">configurable</a>. See <a href="/reference/be/functions#select">build encyclopedia</a> for details.(Õ
configuration_field∑
W
fragmentIThe name of a configuration fragment which contains the late-bound value.(
J
name@The name of the value to obtain from the configuration fragment.(LateBoundDefault"˘References a late-bound default value for an attribute of type <a href="../toplevel/attr.html#label">label</a>. A value is 'late-bound' if it requires the configuration to be built before determining the value. Any attribute using this as a value must <a href="https://bazel.build/extending/rules#private-attributes">be private</a>. <p>Example usage: <p>Defining a rule attribute: <br><pre class=language-python>'_foo': attr.label(default=configuration_field(fragment='java', name='toolchain'))</pre><p>Accessing in rule implementation: <br><pre class=language-python>  def _rule_impl(ctx):
    foo_info = ctx.attr._foo
    ...</pre>(ﬂ

visibility˘

Ï

value‡
A list of package specification strings, or a single package specification string.<p>Package specifications follow the same format as for <code><a href='/reference/be/functions#package_group'>package_group</a></code>, except that negative package specifications are not permitted. That is, a specification may have the forms:<ul><li><code>"//foo"</code>: the package <code>//foo</code><li><code>"//foo/..."</code>: the package <code>//foo</code> and all of its subpackages.<li><code>"public"</code> or <code>"private"</code>: all packages or no packages, respectively</ul><p>The "@" syntax is not allowed; all specifications are interpreted relative to the current module's repository.<p>If <code>value</code> is a list of strings, the set of packages granted visibility to this module is the union of the packages represented by each specification. (An empty list has the same effect as <code>private</code>.) If <code>value</code> is a single string, it is treated as if it were the singleton list <code>[value]</code>.<p>Note that the flags <code>--incompatible_package_group_has_public_syntax</code> and <code>--incompatible_fix_package_group_reporoot_syntax</code> have no effect on this argument. The <code>"public"</code> and <code>"private"</code> values are always available, and <code>"//..."</code> is always interpreted as "all packages in the current repository".(NoneType"“<p>Sets the load visibility of the .bzl module currently being initialized.<p>The load visibility of a module governs whether or not other BUILD and .bzl files may load it. (This is distinct from the target visibility of the underlying .bzl source file, which governs whether the file may appear as a dependency of other targets.) Load visibility works at the level of packages: To load a module the file doing the loading must live in a package that has been granted visibility to the module. A module can always be loaded within its own package, regardless of its visibility.<p><code>visibility()</code> may only be called once per .bzl file, and only at the top level, not inside a function. The preferred style is to put this call immediately below the <code>load()</code> statements and any brief logic needed to determine the argument.<p>If the flag <code>--check_bzl_visibility</code> is set to false, load visibility violations will emit warnings but not fail the build.(Ã=
aspectÄ<
À
implementation∂A Starlark function that implements this aspect, with exactly two parameters: <a href="../builtins/Target.html">Target</a> (the target to which the aspect is applied) and <a href="../builtins/ctx.html">ctx</a> (the rule context which the target is created from). Attributes of the target are available via the <code>ctx.rule</code> field. This function is evaluated during the analysis phase for each application of an aspect to a target.(
∏
attr_aspectss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sÆList of attribute names. The aspect propagates along dependencies specified in the attributes of a target with these names. Common values here include <code>deps</code> and <code>exports</code>. The list can also contain a single string <code>"*"</code> to propagate along all dependencies of a target."[]
á
attrs3<a class="anchor" href="../core/dict.html">dict</a>ƒA dictionary declaring all the attributes of the aspect. It maps from an attribute name to an attribute object, like `attr.label` or `attr.string` (see <a href="../toplevel/attr.html">attr</a> module). Aspect attributes are available to implementation function as fields of <code>ctx</code> parameter. <p>Implicit attributes starting with <code>_</code> must have default values, and have type <code>label</code> or <code>label_list</code>. <p>Explicit attributes must have type <code>string</code>, and must use the <code>values</code> restriction. Explicit attributes restrict the aspect to only be used with rules that have attributes of the same name, type, and valid values according to the restriction."{}
ã	
required_providersThis attribute allows the aspect to limit its propagation to only the targets whose rules advertise its required providers. The value must be a list containing either individual providers or lists of providers but not both. For example, <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code> is a valid value while <code>[FooInfo, BarInfo, [BazInfo, QuxInfo]]</code> is not valid.<p>An unnested list of providers will automatically be converted to a list containing one list of providers. That is, <code>[FooInfo, BarInfo]</code> will automatically be converted to <code>[[FooInfo, BarInfo]]</code>.<p>To make some rule (e.g. <code>some_rule</code>) targets visible to an aspect, <code>some_rule</code> must advertise all providers from at least one of the required providers lists. For example, if the <code>required_providers</code> of an aspect are <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code>, this aspect can see <code>some_rule</code> targets if and only if <code>some_rule</code> provides <code>FooInfo</code>, <em>or</em> <code>BarInfo</code>, <em>or</em> both <code>BazInfo</code> <em>and</em> <code>QuxInfo</code>."[]
é
required_aspect_providersÏThis attribute allows this aspect to inspect other aspects. The value must be a list containing either individual providers or lists of providers but not both. For example, <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code> is a valid value while <code>[FooInfo, BarInfo, [BazInfo, QuxInfo]]</code> is not valid.<p>An unnested list of providers will automatically be converted to a list containing one list of providers. That is, <code>[FooInfo, BarInfo]</code> will automatically be converted to <code>[[FooInfo, BarInfo]]</code>. <p>To make another aspect (e.g. <code>other_aspect</code>) visible to this aspect, <code>other_aspect</code> must provide all providers from at least one of the lists. In the example of <code>[[FooInfo], [BarInfo], [BazInfo, QuxInfo]]</code>, this aspect can see <code>other_aspect</code> if and only if <code>other_aspect</code> provides <code>FooInfo</code>, <em>or</em> <code>BarInfo</code>, <em>or</em> both <code>BazInfo</code> <em>and</em> <code>QuxInfo</code>."[]

providesﬂA list of providers that the implementation function must return.<p>It is an error if the implementation function omits any of the types of providers listed here from its return value. However, the implementation function may return additional providers not listed here.<p>Each element of the list is an <code>*Info</code> object returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a>, except that a legacy provider is represented by its string name instead.When a target of the rule is used as a dependency for a target that declares a required provider, it is not necessary to specify that provider here. It is enough that the implementation function returns it. However, it is considered best practice to specify it, even though this is not required. The <a href='../globals/bzl.html#aspect.required_providers'><code>required_providers</code></a> field of an <a href='../globals/bzl.html#aspect'>aspect</a> does, however, require that providers are specified here."[]
∆
requiresw<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Aspect.html">Aspect</a>s=List of aspects required to be propagated before this aspect."[]
‡
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sZList of names of configuration fragments that the aspect requires in target configuration."[]
„
host_fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sXList of names of configuration fragments that the aspect requires in host configuration."[]
›

toolchains7<a class="anchor" href="../core/list.html">sequence</a>ëIf set, the set of toolchains this rule requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination. Toolchains will be found by checking the current platform, and provided to the rule implementation via <code>ctx.toolchain</code>."[]
k
%incompatible_use_toolchain_transition;Deprecated, this is no longer in use and should be removed."False
∞
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>TA description of the aspect that can be extracted by documentation generating tools."None
Ñ
apply_to_generating_rulesﬂIf true, the aspect will, when applied to an output file, instead apply to the output file's generating rule. <p>For example, suppose an aspect propagates transitively through attribute `deps` and it is applied to target `alpha`. Suppose `alpha` has `deps = [':beta_output']`, where `beta_output` is a declared output of a target `beta`. Suppose `beta` has a target `charlie` as one of its `deps`. If `apply_to_generating_rules=True` for the aspect, then the aspect will propagate through `alpha`, `beta`, and `charlie`. If False, then the aspect will propagate only to `alpha`. </p><p>False by default.</p>"False
Ï
exec_compatible_withs<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s[A list of constraints on the execution platform that apply to all instances of this aspect."[]
á
exec_groupsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>¶Dict of execution group name (string) to <a href='../globals/bzl.html#exec_group'><code>exec_group</code>s</a>. If set, allows aspects to run actions on multiple execution platforms within a single instance. See <a href='/reference/exec-groups'>execution groups documentation</a> for more info."None
æ
subrulesy<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Subrule.html">Subrule</a>s3Experimental: list of subrules used by this aspect."[]Aspect"ºCreates a new aspect. The result of this function must be stored in a global value. Please see the <a href="https://bazel.build/rules/aspects">introduction to Aspects</a> for more details.(Â

exec_group∞
›

toolchains7<a class="anchor" href="../core/list.html">sequence</a>ëThe set of toolchains this execution group requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination."[]
¡
exec_compatible_withs<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s0A list of constraints on the execution platform."[]
exec_group"°Creates an <a href='/reference/exec-groups'>execution group</a> which can be used to create actions for a specific execution platform during rule implementation.(‡
Label
	
input("«Converts a label string into a <code>Label</code> object, in the context of the package where the calling <code>.bzl</code> source file lives. If the given value is already a <code>Label</code>, it is returned unchanged.<p>For macros, a related function, <code><a href='../toplevel/native.html#package_relative_label'>native.package_relative_label()</a></code>, converts the input into a <code>Label</code> in the context of the package currently being constructed. Use that function to mimic the string-to-label conversion that is automatically done by label-valued rule attributes.(
macro (Ã)
provideræ!
≤
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>VA description of the provider that can be extracted by documentation generating tools."None
ÿ
fields¡<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s; or <a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>ÉIf specified, restricts the set of allowed fields. <br>Possible values are:<ul>  <li> list of fields:<br>       <pre class="language-python">provider(fields = ['a', 'b'])</pre><p>  <li> dictionary field name -> documentation:<br>       <pre class="language-python">provider(
       fields = { 'a' : 'Documentation for a', 'b' : 'Documentation for b' })</pre></ul>All fields are optional."None
¢
initcallable; or <code>None</code>ÛAn optional callback for preprocessing and validating the provider's field values during instantiation. If <code>init</code> is specified, <code>provider()</code> returns a tuple of 2 elements: the normal provider symbol and a <em>raw constructor</em>.<p>A precise description follows; see <a href='https://bazel.build/extending/rules#custom_initialization_of_providers'>Rules (Custom initialization of providers)</a> for an intuitive discussion and use cases.<p>Let <code>P</code> be the provider symbol created by calling <code>provider()</code>. Conceptually, an instance of <code>P</code> is generated by calling a default constructor function <code>c(*args, **kwargs)</code>, which does the following:<ul><li>If <code>args</code> is non-empty, an error occurs.</li><li>If the <code>fields</code> parameter was specified when <code>provider()</code> was called, and if <code>kwargs</code> contains any key that was not listed in <code>fields</code>, an error occurs.</li><li>Otherwise, <code>c</code> returns a new instance that has, for each <code>k: v</code> entry in <code>kwargs</code>, a field named <code>k</code> with value <code>v</code>.</ul>In the case where an <code>init</code> callback is <em>not</em> given, a call to the symbol <code>P</code> itself acts as a call to the default constructor function <code>c</code>; in other words, <code>P(*args, **kwargs)</code> returns <code>c(*args, **kwargs)</code>. For example,<pre class="language-python">MyInfo = provider()
m = MyInfo(foo = 1)</pre>will straightforwardly make it so that <code>m</code> is a <code>MyInfo</code> instance with <code>m.foo == 1</code>.<p>But in the case where <code>init</code> is specified, the call <code>P(*args, **kwargs)</code> will perform the following steps instead:<ol><li>The callback is invoked as <code>init(*args, **kwargs)</code>, that is, with the exact same positional and keyword arguments as were passed to <code>P</code>.</li><li>The return value of <code>init</code> is expected to be a dictionary, <code>d</code>, whose keys are field name strings. If it is not, an error occurs.</li><li>A new instance of <code>P</code> is generated as if by calling the default constructor with <code>d</code>'s entries as keyword arguments, as in <code>c(**d)</code>.</li></ol><p>NB: the above steps imply that an error occurs if <code>*args</code> or <code>**kwargs</code> does not match <code>init</code>'s signature, or the evaluation of <code>init</code>'s body fails (perhaps intentionally via a call to <a href="../globals/all.html#fail"><code>fail()</code></a>), or if the return value of <code>init</code> is not a dictionary with the expected schema.<p>In this way, the <code>init</code> callback generalizes normal provider construction by allowing positional arguments and arbitrary logic for preprocessing and validation. It does <em>not</em> enable circumventing the list of allowed <code>fields</code>.<p>When <code>init</code> is specified, the return value of <code>provider()</code> becomes a tuple <code>(P, r)</code>, where <code>r</code> is the <em>raw constructor</em>. In fact, the behavior of <code>r</code> is exactly that of the default constructor function <code>c</code> discussed above. Typically, <code>r</code> is bound to a variable whose name is prefixed with an underscore, so that only the current .bzl file has direct access to it:<pre class="language-python">MyInfo, _new_myinfo = provider(init = ...)</pre>"Noneunknown"¸Defines a provider symbol. The provider may be instantiated by calling it, or used directly as a key for retrieving an instance of that provider from a target. Example:<br><pre class="language-python">MyInfo = provider()
...
def _my_library_impl(ctx):
    ...
    my_info = MyInfo(x = 2, y = 3)
    # my_info.x == 2
    # my_info.y == 3
    ...</pre><p>See <a href='https://bazel.build/extending/rules#providers'>Rules (Providers)</a> for a comprehensive guide on how to use providers.<p>Returns a <a href='../builtins/Provider.html'><code>Provider</code></a> callable value if <code>init</code> is not specified.<p>If <code>init</code> is specified, returns a tuple of 2 elements: a <a href='../builtins/Provider.html'><code>Provider</code></a> callable value and a <em>raw constructor</em> callable value. See <a href='https://bazel.build/extending/rules#custom_initialization_of_providers'> Rules (Custom initialization of custom providers)</a> and the discussion of the <code>init</code> parameter below for details.(Àf
ruleΩc
À
implementation∂the Starlark function implementing this rule, must have exactly one parameter: <a href="../builtins/ctx.html">ctx</a>. The function is called during the analysis phase for each instance of the rule. It can access the attributes provided by the user. It must create actions to generate all the declared outputs.(
ò
test3<a class="anchor" href="../core/bool.html">bool</a>—Whether this rule is a test rule, that is, whether it may be the subject of a <code>blaze test</code> command. All test rules are automatically considered <a href='#rule.executable'>executable</a>; it is unnecessary (and discouraged) to explicitly set <code>executable = True</code> for a test rule. The value defaults to <code>False</code>. See the <a href='https://bazel.build/extending/rules#executable_rules_and_test_rules'> Rules page</a> for more information."unbound
œ
attrs3<a class="anchor" href="../core/dict.html">dict</a>ådictionary to declare all the attributes of the rule. It maps from an attribute name to an attribute object (see <a href="../toplevel/attr.html">attr</a> module). Attributes starting with <code>_</code> are private, and can be used to add an implicit dependency on a label. The attribute <code>name</code> is implicitly added and must not be specified. Attributes <code>visibility</code>, <code>deprecation</code>, <code>tags</code>, <code>testonly</code>, and <code>features</code> are implicitly added and cannot be overridden. Most rules need only a handful of attributes. To limit memory usage, the rule function imposes a cap on the size of attrs."{}
Ä
outputsâ<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>; or <a class="anchor" href="../core/function.html">function</a>‚<b>Deprecated</b>. This parameter is deprecated and will be removed soon. Please do not depend on it. It is <i>disabled</i> with <code>---incompatible_no_rule_outputs_param</code>. Use this flag to verify your code is compatible with its imminent removal. <br>This parameter has been deprecated. Migrate rules to use <code>OutputGroupInfo</code> or <code>attr.output</code> instead. <p>A schema for defining predeclared outputs. Unlike <a href='../toplevel/attr.html#output'><code>output</code></a> and <a href='../toplevel/attr.html#output_list'><code>output_list</code></a> attributes, the user does not specify the labels for these files. See the <a href='https://bazel.build/extending/rules#files'>Rules page</a> for more on predeclared outputs.<p>The value of this argument is either a dictionary or a callback function that produces a dictionary. The callback works similar to computed dependency attributes: The function's parameter names are matched against the rule's attributes, so for example if you pass <code>outputs = _my_func</code> with the definition <code>def _my_func(srcs, deps): ...</code>, the function has access to the attributes <code>srcs</code> and <code>deps</code>. Whether the dictionary is specified directly or via a function, it is interpreted as follows.<p>Each entry in the dictionary creates a predeclared output where the key is an identifier and the value is a string template that determines the output's label. In the rule's implementation function, the identifier becomes the field name used to access the output's <a href='../builtins/File.html'><code>File</code></a> in <a href='../builtins/ctx.html#outputs'><code>ctx.outputs</code></a>. The output's label has the same package as the rule, and the part after the package is produced by substituting each placeholder of the form <code>"%{ATTR}"</code> with a string formed from the value of the attribute <code>ATTR</code>:<ul><li>String-typed attributes are substituted verbatim.<li>Label-typed attributes become the part of the label after the package, minus the file extension. For example, the label <code>"//pkg:a/b.c"</code> becomes <code>"a/b"</code>.<li>Output-typed attributes become the part of the label after the package, including the file extension (for the above example, <code>"a/b.c"</code>).<li>All list-typed attributes (for example, <code>attr.label_list</code>) used in placeholders are required to have <i>exactly one element</i>. Their conversion is the same as their non-list version (<code>attr.label</code>).<li>Other attribute types may not appear in placeholders.<li>The special non-attribute placeholders <code>%{dirname}</code> and <code>%{basename}</code> expand to those parts of the rule's label, excluding its package. For example, in <code>"//pkg:a/b.c"</code>, the dirname is <code>a</code> and the basename is <code>b.c</code>.</ul><p>In practice, the most common substitution placeholder is <code>"%{name}"</code>. For example, for a target named "foo", the outputs dict <code>{"bin": "%{name}.exe"}</code> predeclares an output named <code>foo.exe</code> that is accessible in the implementation function as <code>ctx.outputs.bin</code>."None
„

executable3<a class="anchor" href="../core/bool.html">bool</a>ñWhether this rule is considered executable, that is, whether it may be the subject of a <code>blaze run</code> command. It defaults to <code>False</code>. See the <a href='https://bazel.build/extending/rules#executable_rules_and_test_rules'> Rules page</a> for more information."unbound
ˆ
output_to_genfilesÿIf true, the files will be generated in the genfiles directory instead of the bin directory. Unless you need it for compatibility with existing rules (e.g. when generating header files for C++), do not set this flag."False
ﬁ
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sXList of names of configuration fragments that the rule requires in target configuration."[]
·
host_fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>sVList of names of configuration fragments that the rule requires in host configuration."[]
¡
_skylark_testable§<i>(Experimental)</i><br/><br/>If true, this rule will expose its actions for inspection by rules that depend on it via an <code>Actions</code> provider. The provider is also available to the rule itself by calling <a href="../builtins/ctx.html#created_actions">ctx.created_actions()</a>.<br/><br/>This should only be used for testing the analysis-time behavior of Starlark rules. This flag may be removed in the future."False
›

toolchains7<a class="anchor" href="../core/list.html">sequence</a>ëIf set, the set of toolchains this rule requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination. Toolchains will be found by checking the current platform, and provided to the rule implementation via <code>ctx.toolchain</code>."[]
k
%incompatible_use_toolchain_transition;Deprecated, this is no longer in use and should be removed."False
Æ
docM<a class="anchor" href="../core/string.html">string</a>; or <code>None</code>RA description of the rule that can be extracted by documentation generating tools."None

providesﬂA list of providers that the implementation function must return.<p>It is an error if the implementation function omits any of the types of providers listed here from its return value. However, the implementation function may return additional providers not listed here.<p>Each element of the list is an <code>*Info</code> object returned by <a href='../globals/bzl.html#provider'><code>provider()</code></a>, except that a legacy provider is represented by its string name instead.When a target of the rule is used as a dependency for a target that declares a required provider, it is not necessary to specify that provider here. It is enough that the implementation function returns it. However, it is considered best practice to specify it, even though this is not required. The <a href='../globals/bzl.html#aspect.required_providers'><code>required_providers</code></a> field of an <a href='../globals/bzl.html#aspect'>aspect</a> does, however, require that providers are specified here."[]
Ì
exec_compatible_withs<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s\A list of constraints on the execution platform that apply to all targets of this rule type."[]
∂
analysis_testùIf true, then this rule is treated as an analysis test. <p>Note: Analysis test rules are primarily defined using infrastructure provided in core Starlark libraries. See <a href="https://bazel.build/rules/testing#testing-rules">Testing</a> for guidance. <p>If a rule is defined as an analysis test rule, it becomes allowed to use configuration transitions defined using <a href="#analysis_test_transition">analysis_test_transition</a> on its attributes, but opts into some restrictions: <ul><li>Targets of this rule are limited in the number of transitive dependencies they may have. <li>The rule is considered a test rule (as if <code>test=True</code> were set). This supersedes the value of <code>test</code></li> <li>The rule implementation function may not register actions. Instead, it must register a pass/fail result via providing <a href='../providers/AnalysisTestResultInfo.html'>AnalysisTestResultInfo</a>.</li></ul>"False
‡
build_setting]<a class="anchor" href="../builtins/BuildSetting.html">BuildSetting</a>; or <code>None</code>ÈIf set, describes what kind of <a href='/rules/config#user-defined-build-settings'><code>build setting</code></a> this rule is. See the <a href='../toplevel/config.html'><code>config</code></a> module. If this is set, a mandatory attribute named "build_setting_default" is automatically added to this rule, with a type corresponding to the value passed in here."None
y
cfglIf set, points to the configuration transition the rule will apply to its own configuration before analysis."None
É
exec_groupsI<a class="anchor" href="../core/dict.html">dict</a>; or <code>None</code>¢Dict of execution group name (string) to <a href='../globals/bzl.html#exec_group'><code>exec_group</code>s</a>. If set, allows rules to run actions on multiple execution platforms within a single target. See <a href='/reference/exec-groups'>execution groups documentation</a> for more info."None
¢

initializerå
Experimental: the Stalark function initializing the attributes of the rule. <p>The function is called at load time for each instance of the rule. It's called with <code>name</code> and the values of public attributes defined bythe rule (not with generic attributes, for example <code>tags</code>). <p>It has to return a dictionary from the attribute names to the desired values. The attributes that are not returned are unaffected. Returning <code>None</code> as value results in using the default value specified in the attribute definition. <p>Initializers are evaluated before the default values specified in an attribute definition. Consequently, if a parameter in the initializer's signature contains a default values, it overwrites the default from the attribute definition (except if returning <code>None</code>). <p>Similarly, if a parameter in the initializer's signature doesn't have a default, the parameter will become mandatory. It's a good practice to omit default/mandatory settings on an attribute definition in such cases. <p>It's a good practice to use <code>**kwargs</code> for attributes that are not handled.<p>In case of extended rules, all initializers are called proceeding from child to ancestors. Each initializer is passed only the public attributes it knows about."None
Ñ
parentÛExperimental: the Stalark rule that is extended. When set the public attributes are merged as well as advertised providers. The rule matches <code>executable</code> and <code>test</code> from the parent. Values of <code>fragments</code>, <code>toolchains</code>, <code>exec_compatible_with</code>, and <code>exec_groups</code> are merged. Legacy or deprecated parameters may not be set. Incoming configuration transition <code>cfg</code> of parent is applied after thisrule's incoming configuration."None
°

extendable√<a class="anchor" href="../core/bool.html">bool</a>; or <a class="anchor" href="../builtins/Label.html">Label</a>; or <a class="anchor" href="../core/string.html">string</a>; or <code>None</code>∆Experimental: A label of an allowlist defining which rules can extending this rule. It can be set also to True/False to always allow/disallow extending. Bazel defaults to always allowing extensions."None
º
subrulesy<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Subrule.html">Subrule</a>s1Experimental: List of subrules used by this rule."[]callable"ÄCreates a new rule, which can be called from a BUILD file or a macro to create targets.<p>Rules must be assigned to global variables in a .bzl file; the name of the global variable is the rule's name.<p>Test rules are required to have a name ending in <code>_test</code>, while all other rules must not have this suffix. (This restriction applies only to rules, not to their targets.)(›
subruleœ
Ä
implementation;<a class="anchor" href="../core/function.html">function</a>/The Starlark function implementing this subrule(
π
attrs3<a class="anchor" href="../core/dict.html">dict</a>ˆA dictionary to declare all the (private) attributes of the subrule. <p/>Subrules may only have private attributes that are label-typed (i.e. label or label-list). The resolved values corresponding to these labels are automatically passed by Bazel to the subrule's implementation function as named arguments (thus the implementation function is required to accept named parameters matching the attribute names). The types of these values will be: <ul><li><code>FilesToRunProvider</code> for label attributes with <code>executable=True</code></li><li><code>File</code> for label attributes with <code>allow_single_file=True</code></li><li><code>Target</code> for all other label attributes</li><li><code>[Target]</code> for all label-list attributes</li></ul>"{}
‰

toolchains7<a class="anchor" href="../core/list.html">sequence</a>òIf set, the set of toolchains this subrule requires. The list can contain String, Label, or StarlarkToolchainTypeApi objects, in any combination. Toolchains will be found by checking the current platform, and provided to the subrule implementation via <code>ctx.toolchains</code>."[]
·
	fragmentss<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../core/string.html">string</a>s[List of names of configuration fragments that the subrule requires in target configuration."[]
π
subrulesy<a class="anchor" href="../core/list.html">sequence</a> of <a class="anchor" href="../builtins/Subrule.html">Subrule</a>s.List of other subrules needed by this subrule."[]Subrule"~Constructs a new instance of a subrule. The result of this function must be stored in a global variable before it can be used.(Œ
attrattr"ΩThis is a top-level module for defining the attribute schemas of a rule or aspect. Each function returns an object representing the schema of a single attribute. These objects are used as the values of the <code>attrs</code> dictionary argument of <a href="../globals/bzl.html#rule"><code>rule()</code></a> and <a href="../globals/bzl.html#aspect"><code>aspect()</code></a>.<p>See the Rules page for more on <a href='https://bazel.build/extending/rules#attributes'>defining</a> and <a href='https://bazel.build/extending/rules#implementation_function'>using</a> attributes.(ã
struct

**kwargs8struct"ÊCreates an immutable struct using the keyword arguments as attributes. It is used to group multiple values together. Example:<br><pre class="language-python">s = struct(x = 2, y = 3)
return s.x + getattr(s, "y")  # returns 5</pre>(π
OutputGroupInfo

**kwargs8OutputGroupInfo"ÇInstantiate this provider with <br><pre class=language-python>OutputGroupInfo(group1 = &lt;files&gt;, group2 = &lt;files&gt;...)</pre>See <a href="https://bazel.build/extending/rules#requesting_output_files">Requesting output files </a> for more information.([
ActionsActions"E<b>Deprecated and subject to imminent removal. Please do not use.</b>(≤
DefaultInfos

files"None

runfiles"None

data_runfiles"None

default_runfiles"None


executable"NoneDefaultInfo",<p>The <code>DefaultInfo</code> constructor.(\
RunEnvironmentInfoD

environment"{}

inherited_environment"[]RunEnvironmentInfo((R
CcSharedLibraryInfoCcSharedLibraryInfo"$The type of the Starlark None value.(Z
CcSharedLibraryHintInfoCcSharedLibraryHintInfo"$The type of the Starlark None value.(J
cc_proto_aspectcc_proto_aspect"$The type of the Starlark None value.(Ç
+experimental_java_library_export_do_not_use+experimental_java_library_export_do_not_use"$The type of the Starlark None value.(B
py_internalpy_internal"$The type of the Starlark None value.(ê
PackageSpecificationInfoPackageSpecificationInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(g
config_commonconfig_common"EFunctions for Starlark to interact with Blaze's configurability APIs.(‰
configconfig"œThis is a top-level module for creating configuration transitions and build setting descriptors which describe what kind of build setting (if any) a rule is. <p>ex: the following rule is marked as a build setting by setting the <code>build_setting</code> parameter of the <code>rule()</code> function. Specifically it is a build setting of type <code>int</code> and is a <code>flag</code> which means this build setting is callable on the command line.<br><pre class=language-python>  my_rule = rule(
    implementation = _impl,
    build_setting = config.int(flag = True),
    ...
  )</pre>(∆
analysis_test_transitionÙ
Â
settings÷A dictionary containing information about configuration settings which should be set by this configuration transition. Keys are build setting labels and values are their new post-transition values. All other settings are unchanged. Use this to declare specific configuration settings that an analysis test requires to be set in order to pass.(
transition"∞<p> Creates a configuration transition to be applied on an analysis-test rule's dependencies. This transition may only be applied on attributes of rules with <code>analysis_test = True</code>. Such rules are restricted in capabilities (for example, the size of their dependency tree is limited), so transitions created using this function are limited in potential scope as compared to transitions created using <a href="../builtins/transition.html"><code>transition()</code></a>. <p>This function is primarily designed to facilitate the <a href="https://bazel.build/rules/testing">Analysis Test Framework</a> core library. See its documentation (or its implementation) for best practices.(¢

transition-

implementation(


inputs(

outputs("‚A transition that reads a set of input build settings and writes a set of output build settings.<p>Example:</p><p><pre class="language-python">
def _transition_impl(settings, attr):
    # This transition just reads the current CPU value as a demonstration.
    # A real transition could incorporate this into its followup logic.
    current_cpu = settings["//command_line_option:cpu"]
    return {"//command_line_option:compilation_mode": "dbg"}

build_in_debug_mode = transition(
    implementation = _transition_impl,
    inputs = ["//command_line_option:cpu"],
    outputs = ["//command_line_option:compilation_mode"],
)</pre></p><p>For more details see <a href="https://bazel.build/rules/config#user-defined-transitions">here</a>.</p>(`
platform_commonplatform_common":Functions for Starlark to interact with the platform APIs.(Ú
	ProtoInfo	ProtoInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(˝
proto_commonproto_common"‹Utilities for protocol buffers. <p>Please consider using <code>load("@rules_proto//proto:defs.bzl", "proto_common")</code> to load this symbol from <a href="https://github.com/bazelbuild/rules_proto">rules_proto</a>.</p>(ì
proto_common_do_not_useproto_common_do_not_use"‹Utilities for protocol buffers. <p>Please consider using <code>load("@rules_proto//proto:defs.bzl", "proto_common")</code> to load this symbol from <a href="https://github.com/bazelbuild/rules_proto">rules_proto</a>.</p>(É
ProtoRegistryAspectProtoRegistryAspect"‘For more information about Aspects, please consult the <a href="../globals/bzl.html#aspect">documentation of the aspect function</a> or the <a href="https://bazel.build/rules/aspects">introduction to Aspects</a>.(ä
ProtoRegistryProviderProtoRegistryProvider"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(>
	cc_common	cc_common"$The type of the Starlark None value.(á
CcInfoU

compilation_context"None

linking_context"None

debug_context"NoneCcInfo"$The <code>CcInfo</code> constructor.(®
DebugPackageInfob

target_label(

stripped_file"None

unstripped_file(

dwp_file"NoneDebugPackageInfo".The <code>DebugPackageInfo</code> constructor.(0
CcToolchainConfigInfoCcToolchainConfigInfo(B
java_commonjava_common"$The type of the Starlark None value.(<
JavaInfoJavaInfo"$The type of the Starlark None value.(H
JavaPluginInfoJavaPluginInfo"$The type of the Starlark None value.(o
ProguardSpecProvider!
	
specs(ProguardSpecProvider"2The <code>ProguardSpecProvider</code> constructor.(‡
android_commonandroid_common"ªDo not use this module. It is intended for migration purposes only. If you depend on it, you will be broken when it is removed.Common utilities and functionality related to Android rules.(Â
AndroidIdeInfo¢

java_package(

manifest(

generated_manifest(

idl_import_root(

idl_srcs(

idl_generated_java_files(

idl_source_jar(

idl_class_jar(

defines_android_resources(

resource_jar(

resource_apk(


signed_apk(

aar(

apks_under_test(

native_libs(",The <code>AndroidIdeInfo</code> constructor.(‚
ApkInfoÆ


signed_apk(

unsigned_apk(


deploy_jar(

coverage_metadata(

merged_manifest(

signing_keys(

signing_lineage(
'
#signing_min_v3_rotation_api_version("$The <code>ApkInfo<code> constructor.(f
AndroidInstrumentationInfo


target("8The <code>AndroidInstrumentationInfo</code> constructor.(^
AndroidDeviceBrokerInfo


type("5The <code>AndroidDeviceBrokerInfo</code> constructor.(†
AndroidResourcesInfo—
	
label(

manifest(
	
r_txt(
 
transitive_android_resources(

direct_android_resources(

transitive_resources(

transitive_manifests(

transitive_aapt2_r_txt(

transitive_symbols_bin(

transitive_compiled_symbols(
 
transitive_static_lib"unbound

transitive_r_txt"unbound

validation_artifacts"unbound"2The <code>AndroidResourcesInfo</code> constructor.(a
AndroidNativeLibsInfo

native_libs("3The <code>AndroidNativeLibsInfo</code> constructor.(Ú
AndroidApplicationResourceInfoè

resource_apk(

resource_java_src_jar(

resource_java_class_jar(

manifest(

resource_proguard_config(

main_dex_proguard_config(

r_txt"None

resources_zip"None

databinding_info"None

build_stamp_jar"None
 
should_compile_java_srcs"True"<The <code>AndroidApplicationResourceInfo</code> constructor.(ß
AndroidBinaryNativeLibsInfoK

native_libs(

native_libs_name"None

transitive_native_libs"None"9The <code>AndroidBinaryNativeLibsInfo</code> constructor.(¶
AndroidSdkInfo„

build_tools_version(

framework_aidl(

aidl_lib(

android_jar(

sourceProperties(

shrinked_android_jar(

main_dex_classes(

adb(

dx(

main_dex_list_creator(

aidl(

aapt(
	
aapt2(

apk_builder(


apk_signer(

proguard(

zipalign(

system"None
&
legacy_main_dex_list_generator"None

dexdump"None",The <code>AndroidSdkInfo</code> constructor.(Ç
AndroidManifestInfo6

manifest(

package(

exports_manifest"False"1The <code>AndroidManifestInfo</code> constructor.(Û
AndroidAssetsInfo™
	
label(

validation_result(

direct_parsed_assets(

transitive_parsed_assets(

transitive_assets(

transitive_symbols(

transitive_compiled_symbols("/The <code>AndroidAssetsInfo</code> constructor.(õ
AndroidLibraryAarInfoH

aar(

manifest(

aars_from_deps(

defines_local_resources("6The <code>AndroidLibraryAarInfoApi</code> constructor.(f
AndroidProguardInfo

local_proguard_specs("1The <code>AndroidProguardInfo</code> constructor.(π
AndroidIdlInfow

transitive_idl_import_roots(

transitive_idl_imports(

transitive_idl_jars(

transitive_idl_preprocessed(",The <code>AndroidIdlInfo</code> constructor.(_
AndroidPreDexJarInfo

pre_dex_jar("2The <code>AndroidPreDexJarInfo</code> constructor.(_
AndroidCcLinkParamsInfo
	
store("5The <code>AndroidCcLinkParamsInfo</code> constructor.(Ü
DataBindingV2InfoΩ

setter_store_file"None

class_info_file"None

br_file"None

label"None

java_package"None
&
 databinding_v2_providers_in_deps"[]
)
#databinding_v2_providers_in_exports"[]"/The <code>DataBindingV2Info</code> constructor.(|
&AndroidLibraryResourceClassJarProvider


jars("DThe <code>AndroidLibraryResourceClassJarProvider</code> constructor.(c
AndroidFeatureFlagSet
	
flags(";The <code>AndroidFeatureFlagSetProvider</code> constructor.(b
ProguardMappingInfo

proguard_mapping("1The <code>ProguardMappingInfo</code> constructor.(∑
AndroidBinaryDatah

resource_apk(

resource_proguard_config(

resources_info(

assets_info(

manifest_info("6The <code>AndroidBinaryDataInfoApi</code> constructor.(ë
BaselineProfileProvider=
	
files(

art_profile_zip"NoneBaselineProfileProvider"5The <code>BaselineProfileProvider</code> constructor.(Ø
!AndroidNeverLinkLibrariesProviderG
"
transitive_neverlink_libraries(!AndroidNeverLinkLibrariesProvider"?The <code>AndroidNeverLinkLibrariesProvider</code> constructor.(g
AndroidOptimizedJarInfo

optimized_jar("5The <code>AndroidOptimizedJarInfo</code> constructor.(ç
AndroidDexInfo 


deploy_jar(

final_classes_dex_zip(

filtered_deploy_jar"None
!
final_proguard_output_map"None

java_resource_jar"None
"
shuffled_java_resource_jar"None

rex_output_package_map"None",The <code>AndroidDexInfo</code> constructor.(Ï
AndroidOptimizationInfoî

optimized_jar"None

mapping"None

seeds"None

library_jar"None

config"None

usage"None

proto_mapping"None
!
rewritten_startup_profile"None
(
 rewriten_merged_baseline_profile"None

optimized_resource_apk"None

shrunk_resource_apk"None

shrunk_resource_zip"None

resource_shrinker_log"None
$
resource_optimization_config"None
$
resource_path_shortening_map"None"8The <code>AndroidOptimizationInfoApi</code> constructor.(Ï
PyInfoPyInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(˙
PyRuntimeInfoPyRuntimeInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(¯
PyWrapCcInfoPyWrapCcInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(å
PyCcLinkParamsProviderPyCcLinkParamsProvider"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(m
apple_commonapple_common"MFunctions for Starlark to access internals of the apple rule implementations.(S
testingtesting"=Helper methods for Starlark to access testing infrastructure.(a
coverage_commoncoverage_common";Helper functions to access coverage-related infrastructure.(ä
InstrumentedFilesInfoInstrumentedFilesInfo"◊A constructor for simple value objects, known as provider instances.<br>This value has a dual purpose:  <ul>     <li>It is a function that can be called to construct 'struct'-like values:<pre class="language-python">DataInfo = provider()
d = DataInfo(x = 2, y = 3)
print(d.x + d.y) # prints 5</pre>     Note: Some providers, defined internally, do not allow instance creation     </li>     <li>It is a <i>key</i> to access a provider instance on a        <a href="../builtins/Target.html">Target</a><pre class="language-python">DataInfo = provider()
def _rule_impl(ctx)
  ... ctx.attr.dep[DataInfo]</pre>     </li>  </ul>Create a new <code>Provider</code> using the <a href="../globals/bzl.html#provider">provider</a> function.(,
AnalysisFailureInfoAnalysisFailureInfo(Ñ
AnalysisTestResultInfo2

success(

message(AnalysisTestResultInfo"4The <code>AnalysisTestResultInfo</code> constructor.(—ù
	cc_binary¿ù

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

M
additional_linker_inputs-List of <a href="/concepts/labels">labels</a>"[]
´
args¢<p>
  List of strings; subject to
  <a href="${link make-variables#predefined_label_variables}">$(location)</a> and
  <a href="${link make-variables}">"Make variable"</a> substitution, and
  <a href="#sh-tokenization">Bourne shell tokenization</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
Command line arguments that Bazel will pass to the target when it is executed
either by the <code>run</code> command or as a test. These arguments are
passed before the ones that are specified on the <code>bazel run</code> or
<code>bazel test</code> command line.
</p>

<p>
<em class="harmful">NOTE: The arguments are not passed when you run the target
outside of Bazel (for example, by manually executing the binary in
<code>bazel-bin/</code>).</em>
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>


coptsList of strings"[]

definesList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

A
dynamic_deps-List of <a href="/concepts/labels">labels</a>"[]
Á
envﬂ<p>Dictionary of strings; values are subject to
<a href="${link make-variables#predefined_label_variables}">$(location)</a> and
<a href="${link make-variables}">"Make variable"</a> substitution; default is <code>{}</code></p>

<p>Specifies additional environment variables to set when the target is
  executed by <code>bazel run</code>.
</p>

<p>
  This attribute only applies to native rules, like <code>cc_binary</code>, <code>py_binary</code>,
  and <code>sh_binary</code>.  It does not apply to Starlark-defined executable rules.
</p>

<p>
<em class="harmful">NOTE: The environment variables are not set when you run the target
outside of Bazel (for example, by manually executing the binary in
<code>bazel-bin/</code>).</em>
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>




hdrs_checkString"""

includesList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

`
link_extra_lib$<a href="/concepts/labels">Label</a>"("@bazel_tools//tools/cpp:link_extra_lib"

linkoptsList of strings"[]


linksharedBoolean"False


linkstaticBoolean"True
$
local_definesList of strings"[]
P
malloc$<a href="/concepts/labels">Label</a>" "@bazel_tools//tools/cpp:malloc"

nocoptsString"""
Ê
output_licenses“<p>List of strings; default is <code>[]</code></p>

<p>
The licenses of the output files that this binary generates.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


B
reexport_deps-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>


stampInteger"-1
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

:
win_def_file$<a href="/concepts/labels">Label</a>"None(¬à
	cc_import±à

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



includesList of strings"[]
?
interface_library$<a href="/concepts/labels">Label</a>"None

linkoptsList of strings"[]
<
objects-List of <a href="/concepts/labels">labels</a>"[]
@
pic_objects-List of <a href="/concepts/labels">labels</a>"[]
@
pic_static_library$<a href="/concepts/labels">Label</a>"None
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

<
shared_library$<a href="/concepts/labels">Label</a>"None
<
static_library$<a href="/concepts/labels">Label</a>"None
!
system_providedBoolean"False
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(Ôê

cc_library›ê

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]
O
additional_compiler_inputs-List of <a href="/concepts/labels">labels</a>"[]
M
additional_linker_inputs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>


coptsList of strings"[]

definesList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


à

hdrs_checkString"r<unknown object com.google.devtools.build.lib.rules.cpp.CcStarlarkInternal$DefaultHdrsCheckBuiltinComputedDefault>
H
implementation_deps-List of <a href="/concepts/labels">labels</a>"[]

include_prefixString"""

includesList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


linkoptsList of strings"[]
7
	linkstamp$<a href="/concepts/labels">Label</a>"None


linkstaticBoolean"False
$
local_definesList of strings"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

"
strip_include_prefixString"""
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

A
textual_hdrs-List of <a href="/concepts/labels">labels</a>"[]
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

:
win_def_file$<a href="/concepts/labels">Label</a>"None(ë{
cc_proto_library˙z

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(˝~
cc_shared_libraryÂ~

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

M
additional_linker_inputs-List of <a href="/concepts/labels">labels</a>"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

A
dynamic_deps-List of <a href="/concepts/labels">labels</a>"[]
‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

M
;experimental_disable_topo_sort_do_not_use_remove_before_7_0Boolean"False
%
exports_filterList of strings"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

:
roots-List of <a href="/concepts/labels">labels</a>"[]

shared_lib_nameString"""
"
static_depsList of strings"[]
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

&
user_link_flagsList of strings"[]
ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

:
win_def_file$<a href="/concepts/labels">Label</a>"None(éä
java_import˚â

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

"
add_exportsList of strings"[]
 
	add_opensList of strings"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

"
constraintsList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

<
exports-List of <a href="/concepts/labels">labels</a>"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


7
jars-List of <a href="/concepts/labels">labels</a>(
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


	neverlinkBoolean"False
C
proguard_specs-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

A
runtime_deps-List of <a href="/concepts/labels">labels</a>"[]
4
srcjar$<a href="/concepts/labels">Label</a>"None
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(öé
java_libraryÜé

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

>
	resources-List of <a href="/concepts/labels">labels</a>"[]
"
add_exportsList of strings"[]
 
	add_opensList of strings"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

E
exported_plugins-List of <a href="/concepts/labels">labels</a>"[]
<
exports-List of <a href="/concepts/labels">labels</a>"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


 
	javacoptsList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


	neverlinkBoolean"False
<
plugins-List of <a href="/concepts/labels">labels</a>"[]
C
proguard_specs-List of <a href="/concepts/labels">labels</a>"[]
#
resource_strip_prefixString"""
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

A
runtime_deps-List of <a href="/concepts/labels">labels</a>"[]
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(ò{
java_lite_proto_library˙z

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(ﬁ}
java_proto_library≈}

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(–z
java_package_configurationØz

name(
˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


 
	javacoptsList of strings"[]
&
output_licensesList of strings"[]
=
packages-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(πç
java_plugin¶ç

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

>
	resources-List of <a href="/concepts/labels">labels</a>"[]
"
add_exportsList of strings"[]
 
	add_opensList of strings"[]
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



generates_apiBoolean"False
 
	javacoptsList of strings"[]
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>


	neverlinkBoolean"False
&
output_licensesList of strings"[]
<
plugins-List of <a href="/concepts/labels">labels</a>"[]

processor_classString"""
C
proguard_specs-List of <a href="/concepts/labels">labels</a>"[]
#
resource_strip_prefixString"""
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(úw
java_runtimeâw

name(
è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

9
default_cds$<a href="/concepts/labels">Label</a>"None
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


B
hermetic_srcs-List of <a href="/concepts/labels">labels</a>"[]
I
hermetic_static_libs-List of <a href="/concepts/labels">labels</a>"[]
2
java$<a href="/concepts/labels">Label</a>"None

	java_homeString"""
8

lib_ct_sym$<a href="/concepts/labels">Label</a>"None
9
lib_modules$<a href="/concepts/labels">Label</a>"None
&
output_licensesList of strings"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>


versionInteger"0
ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(Ù{
j2objc_libraryﬂ{

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

$
entry_classesList of strings"[]
‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


=
jre_deps-List of <a href="/concepts/labels">labels</a>"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(°~
objc_importè~

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
;
archives-List of <a href="/concepts/labels">labels</a>(
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



includesList of strings"[]
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

!

sdk_dylibsList of strings"[]
%
sdk_frameworksList of strings"[]
#
sdk_includesList of strings"[]
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

A
textual_hdrs-List of <a href="/concepts/labels">labels</a>"[]
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

*
weak_sdk_frameworksList of strings"[](°ç
objc_libraryçç

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

9
hdrs-List of <a href="/concepts/labels">labels</a>"[]


alwayslinkBoolean"False
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>


coptsList of strings"[]

definesList of strings"[]
¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

 
enable_modulesBoolean"False
‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>


H
implementation_deps-List of <a href="/concepts/labels">labels</a>"[]

includesList of strings"[]

linkoptsList of strings"[]
8

module_map$<a href="/concepts/labels">Label</a>"None

module_nameString"""
A
non_arc_srcs-List of <a href="/concepts/labels">labels</a>"[]
1
pch$<a href="/concepts/labels">Label</a>"None
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

!

sdk_dylibsList of strings"[]
%
sdk_frameworksList of strings"[]
#
sdk_includesList of strings"[]

stampBoolean"False
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

A
textual_hdrs-List of <a href="/concepts/labels">labels</a>"[]
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>

*
weak_sdk_frameworksList of strings"[](¢ã
proto_libraryçã

name(
À

deps¬
<a name="common.deps"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Dependencies for this target. Generally should only list rule targets. (Though
some rules permit files to be listed directly in <code>deps</code>, this
should be avoided when possible.)
</p>

<p>
Language-specific rules generally limit the listed targets to those with
specific <a href="https://bazel.build/extending/rules#providers">providers</a>.</p>

<p>
The precise semantics of what it means for a target to depend on another using
<code>deps</code> are specific to the kind of rule, and the rule-specific
documentation goes into more detail. For rules which process source code,
<code>deps</code> generally specifies code dependencies used by the code in
<a href="#typical.srcs"><code>srcs</code></a>.
</p>

<p>
Most often, a <code>deps</code> dependency is used to allow one module to use
symbols defined in another module written in the same programming language and
separately compiled.  Cross-language dependencies are also permitted in many
cases: For example, a <code>java_library</code> target may depend on C++ code
in a <code>cc_library</code> target, by listing the latter in the
<code>deps</code> attribute.  See the definition of
<a href="${link build-ref#deps}">dependencies</a>
for more information.
</p>

è
srcsÜ<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files processed or included by this rule. Generally lists files directly, but
may list rule targets (like <code>filegroup</code> or <code>genrule</code>) to
include their default outputs.
</p>

<p>
Language-specific rules often require that the listed files have particular
file extensions.
</p>

˜
dataÓ<a name="common.data"></a>
<p>List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code></p>

<p>
Files needed by this rule at runtime. May list file or rule targets. Generally
allows any target.
</p>

<p>
The default outputs and runfiles of targets in the <code>data</code> attribute
should appear in the <code>*.runfiles</code> area of any executable which is
output by or has a runtime dependency on this target. This may include data
files or binaries used when this target's
<a href="#typical.srcs"><code>srcs</code></a> are executed. See the
<a href="${link build-ref#data}">data dependencies</a>
section for more information about how to depend on and use data files.
</p>

<p>
New rules should define a <code>data</code> attribute if they process
inputs which might use other inputs at runtime. Rules' implementation functions
must also <a href="https://bazel.build/rules/rules#runfiles">populate the target's
runfiles</a> from the outputs and runfiles of any <code>data</code> attribute,
as well as runfiles from any dependency attribute which provides either
source code or runtime dependencies.
</p>

;
allow_exports$<a href="/concepts/labels">Label</a>"None
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

<
exports-List of <a href="/concepts/labels">labels</a>"[]
ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



import_prefixString"""
»
licensesª<a name="common.licenses"></a>
<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>["none"]</code></p>

<p>
  A list of license-type strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

"
strip_import_prefixString""/"
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(≈t
proto_lang_toolchain™t

name(
I
allowlist_different_package$<a href="/concepts/labels">Label</a>"None
G
blacklisted_protos-List of <a href="/concepts/labels">labels</a>"[]

command_lineString(
æ
compatible_with™<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, in addition to
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system, which lets users declare which
targets can and cannot depend on each other. For example, externally deployable
binaries shouldn't depend on libraries with company-secret code. See
<a href="https://github.com/bazelbuild/bazel/blob/master/src/main/java/com/google/devtools/build/lib/analysis/constraints/ConstraintSemantics.java#L46">
ConstraintSemantics</a> for details.
</p>

¸	
deprecationÏ	<p>String; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>None</code></p>

<p>
An explanatory warning message associated with this target.
Typically this is used to notify users that a target has become obsolete,
or has become superseded by another rule, is private to a package, or is
perhaps considered harmful for some reason. It is a good idea to include
some reference (like a webpage, a bug number or example migration CLs) so
that one can easily find out what changes are required to avoid the message.
If there is a new target that can be used as a drop in replacement, it is a
good idea to just migrate all users of the old target.
</p>

<p>
This attribute has no effect on the way things are built, but it
may affect a build tool's diagnostic output.  The build tool issues a
warning when a rule with a <code>deprecation</code> attribute is
depended upon by a target in another package.
</p>

<p>
Intra-package dependencies are exempt from this warning, so that,
for example, building the tests of a deprecated rule does not
encounter a warning.
</p>

<p>
If a deprecated target depends on another deprecated target, no warning
message is issued.
</p>

<p>
Once people have stopped using it, the target can be removed.
</p>

™
distribsù<p>List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code></p>

<p>
  A list of distribution-method strings to be used for this particular target.

This is part of a deprecated licensing API that Bazel no longer uses. Don't
use this.
</p>

‡
exec_compatible_with«<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_values</a></code>
that must be present in the execution platform for this target. This is in
addition to any constraints already set by the rule type. Constraints are used
to restrict the list of available execution platforms. For more details, see
the description of
  <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>.
  </p>

ó
exec_propertiesÉ<p>Dictionary of strings; default is <code>{}</code></p>

<p> A dictionary of strings that will be added to the <code>exec_properties</code> of a platform selected for this target. See <code>exec_properties</code> of the <a href="platform.html">platform</a> rule.</p>

<p>If a key is present in both the platform and target-level properties, the value will be taken from the target.</p>

ì
featuresÜ<p>List of <i>feature</i> strings; default is <code>[]</code></p>

<p>A feature is string tag that can be enabled or disabled on a target. The
  meaning of a feature depends on the rule itself.</p>

<p>This <code>features</code> attribute is combined with the <a href="${link package}">
package</a> level <code>features</code> attribute. For example, if
the features ["a", "b"] are enabled on the package level, and a target's
<code>features</code> attribute contains ["-a", "c"], the features enabled for the
rule will be "b" and "c".
  <a href="https://github.com/bazelbuild/examples/blob/main/rules/features/BUILD">
    See example</a>.
</p>



mnemonicString"
"GenProto"
 
output_filesString""legacy"
4
plugin$<a href="/concepts/labels">Label</a>"None
 
plugin_format_flagString"""
?
progress_messageString"#"Generating proto_library %{label}"
°
restricted_toè<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
The list of environments this target can be built for, <i>instead</i> of
default-supported environments.
</p>

<p>
This is part of Bazel's constraint system. See
<code><a href="#common.compatible_with">compatible_with</a></code>
for details.
</p>

5
runtime$<a href="/concepts/labels">Label</a>"None
∏-
tagsØ-<p>
  List of strings; <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code>[]</code>
</p>

<p>
  <i>Tags</i> can be used on any rule. <i>Tags</i> on test and
  <code>test_suite</code> rules are useful for categorizing the tests.
  <i>Tags</i> on non-test targets are used to control sandboxed execution of
  <code>genrule</code>s and

<a href="/rules/concepts">Starlark</a>
  actions, and for parsing by humans and/or external tools.
</p>

<p>
  Bazel modifies the behavior of its sandboxing code if it finds the following
  keywords in the <code>tags</code> attribute of any test or <code>genrule</code>
  target, or the keys of <code>execution_requirements</code> for any Starlark
  action.
</p>

<ul>
    <li><code>no-sandbox</code> keyword results in the action or test never being
    sandboxed; it can still be cached or run remotely - use <code>no-cache</code>
    or <code>no-remote</code> to prevent either or both of those.
  </li>
  <li><code>no-cache</code> keyword results in the action or test never being
    cached (locally or remotely). Note: for the purposes of this tag, the disk cache
    is considered a local cache, whereas the HTTP and gRPC caches are considered
    remote. Other caches, such as Skyframe or the persistent action cache, are not
    affected.
  </li>
    <li><code>no-remote-cache</code> keyword results in the action or test never being
    cached remotely (but it may be cached locally; it may also be executed remotely).
    Note: for the purposes of this tag, the disk cache is considered a local cache,
    whereas the HTTP and gRPC caches are considered remote. Other caches, such as
    Skyframe or the persistent action cache, are not affected.
    If a combination of local disk cache and remote cache are used (combined cache),
    it's treated as a remote cache and disabled entirely unless <code>--incompatible_remote_results_ignore_disk</code>
    is set in which case the local components will be used.
  </li>
    <li><code>no-remote-exec</code> keyword results in the action or test never being
    executed remotely (but it may be cached remotely).
  </li>

  <li><code>no-remote</code> keyword prevents the action or test from being executed remotely or
    cached remotely. This is equivalent to using both
    <code>no-remote-cache</code> and <code>no-remote-exec</code>.
      </li>
   <li><code>no-remote-cache-upload</code> keyword disables upload part of remote caching of a spawn.
     it does not disable remote execution.
  </li>
    <li><code>local</code> keyword precludes the action or test from being remotely cached,
    remotely executed, or run inside the sandbox.
    For genrules and tests, marking the rule with the <code>local = True</code>
    attribute has the same effect.
  </li>

    <li><code>requires-network</code> keyword allows access to the external
    network from inside the sandbox.  This tag only has an effect if sandboxing
    is enabled.
  </li>

  <li><code>block-network</code> keyword blocks access to the external
    network from inside the sandbox. In this case, only communication
    with localhost is allowed. This tag only has an effect if sandboxing is
    enabled.
  </li>

  <li><code>requires-fakeroot</code> runs the test or action as uid and gid 0 (i.e., the root
    user). This is only supported on Linux. This tag takes precedence over the
    <code class='flag'>--sandbox_fake_username</code> command-line option.
  </li>
</ul>

<p>
  <i>Tags</i> on tests are generally used to annotate a test's role in your
  debug and release process.  Typically, tags are most useful for C++ and Python
  tests, which lack any runtime annotation ability.  The use of tags and size
  elements gives flexibility in assembling suites of tests based around codebase
  check-in policy.
</p>

<p>
  Bazel modifies test running behavior if it finds the following keywords in the
  <code>tags</code> attribute of the test rule:
</p>

<ul>
  <li><code>exclusive</code> will force the test to be run in the
    &quot;exclusive&quot; mode, ensuring that no other tests are running at the
    same time. Such tests will be executed in serial fashion after all build
    activity and non-exclusive tests have been completed. Remote execution is
    disabled for such tests because Bazel doesn't have control over what's
    running on a remote machine.
  </li>

  <li><code>exclusive-if-local</code> will force the test to be run in the
    &quot;exclusive&quot; mode if it is executed locally, but will run the test in parallel if it's
    executed remotely.
  </li>

  <li><code>manual</code> keyword will exclude the target from expansion of target pattern wildcards
    (<code>...</code>, <code>:*</code>, <code>:all</code>, etc.) and <code>test_suite</code> rules
    which do not list the test explicitly when computing the set of top-level targets to build/run
    for the <code>build</code>, <code>test</code>, and <code>coverage</code> commands. It does not
    affect target wildcard or test suite expansion in other contexts, including the
    <code>query</code> command. Note that <code>manual</code> does not imply that a target should
    not be built/run automatically by continuous build/test systems. For example, it may be
    desirable to exclude a target from <code>bazel test ...</code> because it requires specific
    Bazel flags, but still have it included in properly-configured presubmit or continuous test
    runs.

      </li>

  <li><code>external</code> keyword will force test to be unconditionally
    executed (regardless of <code class='flag'>--cache_test_results</code>
    value).
  </li>

  </ul>

See
<a href="${link test-encyclopedia#tag-conventions}">Tag Conventions</a>
 in the Test Encyclopedia for more conventions on tags attached to test targets.

…
target_compatible_withÆ<p>
List of <a href="${link build-ref#labels}">labels</a>; default is <code>[]</code>
</p>

<p>
A list of
<code><a href="platforms-and-toolchains#constraint_value">constraint_value</a></code>s
that must be present in the target platform for this target to be considered
<em>compatible</em>. This is in addition to any constraints already set by the
rule type. If the target platform does not satisfy all listed constraints then
the target is considered <em>incompatible</em>. Incompatible targets are
skipped for building and testing when the target pattern is expanded
(e.g. <code>//...</code>, <code>:all</code>). When explicitly specified on the
command line, incompatible targets cause Bazel to print an error and cause a
build or test failure.
</p>

<p>
Targets that transitively depend on incompatible targets are themselves
considered incompatible. They are also skipped for building and testing.
</p>

<p>
An empty list (which is the default) signifies that the target is compatible
with all platforms.
<p>

<p>
All rules other than <a href="workspace.html">Workspace Rules</a> support this
attribute.
For some rules this attribute has no effect. For example, specifying
<code>target_compatible_with</code> for a
<code><a href="c-cpp.html#cc_toolchain">cc_toolchain</a></code> is not useful.
<p>

<p>
See the
<a href="/docs/platforms#skipping-incompatible-targets">Platforms</a>
page for more information about incompatible target skipping.
</p>

´	
testonlyû	<p>Boolean; <a href="#configurable-attributes">nonconfigurable</a>; default is <code>False</code>
  except for test and test suite targets</p>

<p>
If <code>True</code>, only testonly targets (such as tests) can depend on this target.
</p>

<p>
Equivalently, a rule that is not <code>testonly</code> is not allowed to
depend on any rule that is <code>testonly</code>.
</p>

<p>
Tests (<code>*_test</code> rules)
and test suites (<a href="${link test_suite}">test_suite</a> rules)
are <code>testonly</code> by default.
</p>

<p>
This attribute is intended to mean that the target should not be
contained in binaries that are released to production.
</p>

<p>
Because testonly is enforced at build time, not run time, and propagates
virally through the dependency tree, it should be applied judiciously. For
example, stubs and fakes that
are useful for unit tests may also be useful for integration tests
involving the same binaries that will be released to production, and
therefore should probably not be marked testonly. Conversely, rules that
are dangerous to even link in, perhaps because they unconditionally
override normal behavior, should definitely be marked testonly.
</p>

<
toolchain_type$<a href="/concepts/labels">Label</a>"None
≤

toolchains£<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>; default is <code>[]</code></p>

<p>
  The set of targets whose <a href="${link make-variables}">Make variables</a> this target is
  allowed to access. These targets are either instances of rules that provide
  <code>TemplateVariableInfo</code> or special targets for toolchain types built into Bazel. These
  include:

<ul>
    <li><code>@bazel_tools//tools/cpp:current_cc_toolchain</code>
  <li><code>@bazel_tools//tools/jdk:current_java_runtime</code>
  </ul>

<p>
  Note that this is distinct from the concept of
    <a href="/docs/toolchains#toolchain-resolution">toolchain resolution</a>
    that is used by rule implementations for platform-dependent configuration. You cannot use this
  attribute to determine which specific <code>cc_toolchain</code> or <code>java_toolchain</code> a
  target will use.
</p>

ë

visibilityÇ<p>List of <a href="${link build-ref#labels}">labels</a>;
  <a href="#configurable-attributes">nonconfigurable</a>;
  default is <code><a href="${link package.default_visibility}">default_visibility</a></code> from
  <a href="${link package}">package</a> if specified, or <code>"//visibility:private"</code>
  otherwise</p>

<p>
The <code>visibility</code> attribute on a target controls whether the target
can be used in other packages. See the documentation for
<a href="${link visibility}">visibility</a>.
</p>
(