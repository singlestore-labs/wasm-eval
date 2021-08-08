ONNX is an open format built to represent machine learning models. ONNX defines a common set of operators - the building blocks of machine learning and deep learning models - and a common file format to enable AI developers to use models with a variety of frameworks, tools, runtimes, and compilers.

Develop in your preferred framework without worrying about downstream inferencing implications. ONNX enables you to use your preferred framework with your chosen inference engine.


https://pytorch.org/tutorials/advanced/super_resolution_with_onnxruntime.html



https://docs.microsoft.com/en-us/azure/machine-learning/concept-onnx



Microsoft and a community of partners created ONNX as an open standard for representing machine learning models. Models from many frameworks including TensorFlow, PyTorch, SciKit-Learn, Keras, Chainer, MXNet, MATLAB, and SparkML can be exported or converted to the standard ONNX format. Once the models are in the ONNX format, they can be run on a variety of platforms and devices.

ONNX Runtime is used in high-scale Microsoft services such as Bing, Office, and Azure Cognitive Services. Performance gains are dependent on a number of factors, but these Microsoft services have seen an average 2x performance gain on CPU. In addition to Azure Machine Learning services, ONNX Runtime also runs in other products that support Machine Learning workloads, including:

Windows: The runtime is built into Windows as part of Windows Machine Learning and runs on hundreds of millions of devices.
Azure SQL product family: Run native scoring on data in Azure SQL Edge and Azure SQL Managed Instance.
ML.NET: Run ONNX models in ML.NET.




_start() : Exported automatically when you compile Webassembly as WASI. Initializes the executable, then calls main() - which is expected to return EXIT_SUCCESSon successful execution, and EXIT_FAILURE on failure.
pred() : Called to run the actual task. Return EXIT_SUCCESSon successful execution, and EXIT_FAILURE on failure.
malloc_buffer(length) : Called to allocate memory for the input-data. Returns a pointer to the start of the allocated chunk of memory.
free_buffer(location) : Called to free up allocated memory again.
get_out_len() : Called to retrieve the length of the output-data. Returns an unsigned integer.
get_out_loc() : Called to retrieve the location of the output-data. Returns an unsigned integer that represents (points to) to the start position of the output string in wasm's linear memory.-