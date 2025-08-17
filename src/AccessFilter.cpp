#include "AccessFilter.hpp"
#include "EventLabel.hpp"
#include "MemAccess.hpp"

bool ::detail::OverlapFilter::operator()(const MemAccessLabel &s) const
{
	return s.getAccess().overlaps(access);
}
