/*
 * GenMC -- Generic Model Checking.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, you can access it online at
 * http://www.gnu.org/licenses/gpl-3.0.html.
 *
 * Author: Michalis Kokologiannakis <michalis@mpi-sws.org>
 */

#ifndef __MEM_ACCESS_HPP__
#define __MEM_ACCESS_HPP__

#include "Error.hpp"
#include "SAddr.hpp"
#include "config.h"

#include <algorithm>
#include <climits>
#include <cstdint>

/*******************************************************************************
 **                             AType Enum
 ******************************************************************************/

/*
 * Represents the type of an access: pointer, signed integer, unsigned integer
 */
enum class AType { Pointer, Signed, Unsigned };

/*******************************************************************************
 **                             AAccess Class
 ******************************************************************************/

/*
 * An AAccess comprises an address, a size and a type
 */
class AAccess {

public:
	AAccess() = delete;
	AAccess(SAddr addr, ASize size, AType type) : addr(addr), size(size), type(type) {}

	[[nodiscard]] auto getAddr() const -> SAddr { return addr; }
	[[nodiscard]] auto getSize() const -> ASize { return size; }
	[[nodiscard]] auto getType() const -> AType { return type; }

	[[nodiscard]] auto isPointer() const -> bool { return getType() == AType::Pointer; }
	[[nodiscard]] auto isUnsigned() const -> bool { return getType() == AType::Unsigned; }
	[[nodiscard]] auto isSigned() const -> bool { return getType() == AType::Signed; }

	/* Whether the access contains a given address */
	[[nodiscard]] auto contains(SAddr addr) const -> bool
	{
		if (!getAddr().sameStorageAs(addr))
			return false;
		return getAddr() <= addr && addr < getAddr() + getSize();
	}

	/* Whether the access overlaps with another access */
	[[nodiscard]] auto overlaps(const AAccess &other) const -> bool
	{
		if (!getAddr().sameStorageAs(other.getAddr()))
			return false;
		return getAddr() + getSize() > other.getAddr() &&
		       getAddr() < other.getAddr() + other.getSize();
	}

	inline bool operator==(const AAccess &other) const = default;

	friend llvm::raw_ostream& operator<<(llvm::raw_ostream& s, const AAccess &a);

	class WordFragment : public std::set<int> {
		public:
			WordFragment() = default;
			WordFragment(AAccess access) {
				auto first = access.getAddr().getByte();
				for (auto i = 0; i < access.getSize(); ++i)
					insert(first + i);
			}

			WordFragment subtract(const WordFragment &other) const {
				WordFragment diff;
				std::set_difference(begin(), end(), other.begin(),
						other.end(), std::inserter(diff, diff.end()));
				return diff;
			}

			WordFragment intersect(const WordFragment &other) const {
				WordFragment inter;
				std::set_intersection(begin(), end(), other.begin(),
						other.end(), std::inserter(inter, inter.end()));
				return inter;
			}
	};

	static std::vector<AAccess> getAccesses(SAddr word, WordFragment fragment, AType type = AType::Unsigned)
	{
		BUG_ON(word != word.align());
		std::vector<AAccess> ret;
		auto it = fragment.begin(), ie = fragment.end();
		while (it != ie) {
			auto beg = it;
			auto size = 0;
			do {
				auto before = *it;
				it++;
				size++;
			} while(it != ie && (*it == *beg + size));
			ret.push_back(AAccess(word + ASize(*beg), ASize(size), type));
		}
		return ret;
	}

	WordFragment getOverlap(const WordFragment &fragment) const
	{
		return WordFragment(*this).intersect(fragment);
	}


private:
	SAddr addr;
	ASize size;
	AType type;
};

#endif /* __MEM_ACCESS_HPP__ */
