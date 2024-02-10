#pragma once

#ifdef _WIN64
#define MONKEY_WINDOWS 1
#define MONKEY_WINDOWS_64 1
#elif defined(_WIN32)
#define MONKEY_WINDOWS 1
#define MONKEY_WINDOWS_64 0
#else
#define MONKEY_WINDOWS 0
#define MONKEY_WINDOWS_64 0
#endif // !_WIN64

#ifdef __ANDROID__
#define MONKEY_LINUX 1
#define MONKEY_ANDROID 1
#elif defined(__linux__)
#define MONKEY_LINUX 1
#define MONKEY_ANDROID 0
#else
#define MONKEY_LINUX 0
#define MONKEY_ANDROID 0
#endif // !__ANDROID__

#ifdef __APPLE__
#define MONKEY_APPLE 1
#else
#define MONKEY_APPLE 0
#endif // !__APPLE__

#if defined(__clang__) && !defined(__ibmxl__)
#define MONKEY_CLANG_VERSION (__clang_major__ * 100 + __clang_minor__)
#else
#define MONKEY_CLANG_VERSION 0
#endif

#if defined(__GNUC__) && !defined(__clang__) && !defined(__INTEL_COMPILER) && \
  !defined(__NVCOMPILER)
#define MONKEY_GCC_VERSION (__GNUC__ * 100 + __GNUC_MINOR__)
#else
#define MONKEY_GCC_VERSION 0
#endif

#ifdef __ICL
#define MONKEY_ICC_VERSION __ICL
#elif defined(__INTEL_COMPILER)
#define MONKEY_ICC_VERSION __INTEL_COMPILER
#else
#define MONKEY_ICC_VERSION 0
#endif // !__ICL

#ifdef _MSC_VER
#define MONKEY_MSC_VERSION _MSC_VER
#else
#define MONKEY_MSC_VERSION 0
#endif // !_MSC_VER

#ifdef _MSVC_LANG
#define MONKEY_CPLUSPLUS _MSVC_LANG
#else
#define MONKEY_CPLUSPLUS __cplusplus
#endif // !_MSVC_LANG

#ifdef __has_feature
#define MONKEY_HAS_FEATURE(x) __has_feature(x)
#else
#define MONKEY_HAS_FEATURE(x) 0
#endif // !__has_feature

#if defined(__has_include) || MONKEY_ICC_VERSION >= 1600 || MONKEY_MSC_VERSION > 1900
#define MONKEY_HAS_INCLUDE(x) __has_include(x)
#else
#define MONKEY_HAS_INCLUDE(x) 0
#endif

#ifdef __has_cpp_attribute
#define MONKEY_HAS_CPP_ATTRIBUTE(x) __has_cpp_attribute(x)
#else
#define MONKEY_HAS_CPP_ATTRIBUTE(x) 0
#endif // !__has_cpp_attribute

#if MONKEY_WINDOWS
#ifdef MONKEY_LIB_EXPORT
#define MONKEY_API __declspec(dllexport)
#elif defined(MONKEY_SHARED)
#define MONKEY_API __declspec(dllimport)
#endif // !MONKEY_LIB_EXPORT
#endif // !MONKEY_WINDOWS
#ifndef MONKEY_API
#define MONKEY_API
#endif // !MONKEY_API

#define BEGIN_MONKEY_NAMESPACE namespace monkey {
#define END_MONKEY_NAMESPACE }
