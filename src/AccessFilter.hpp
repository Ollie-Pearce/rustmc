#ifndef __ACCESS_FILTER_HPP__
#define __ACCESS_FILTER_HPP__

#include <llvm/ADT/STLExtras.h>

class AAccess;
class MemAccessLabel;

namespace detail {
struct OverlapFilter {
	OverlapFilter() = delete;
	OverlapFilter(const AAccess &a) : access(a) {}

	auto operator()(const MemAccessLabel &s) const -> bool;

private:
	const AAccess &access;
};

/* Custom implementation to distinguish between [non-]overlapping memory accesses */
template <typename IterT>
struct access_filter_iterator : public llvm::filter_iterator<IterT, OverlapFilter> {
public:
	using BaseT = llvm::filter_iterator<IterT, OverlapFilter>;

	access_filter_iterator(IterT it, IterT end, OverlapFilter filter) : BaseT(it, end, filter) {}

	/* We need implicit conversion to the wrapped iterator */
	operator const IterT &() const { return this->wrapped(); }

	auto operator++() -> access_filter_iterator &
	{
		return static_cast<access_filter_iterator &>(BaseT::operator++());
	}
	auto operator++(int) -> access_filter_iterator
	{
		auto tmp = *this;
		BaseT::operator++();
		return tmp;
	}
};
}; // namespace detail

#endif /* __ACCESS_FILTER_HPP__ */
