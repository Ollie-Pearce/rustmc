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

/*******************************************************************************
 * CAUTION: This file is generated automatically by Kater -- DO NOT EDIT.
 *******************************************************************************/

#include "RADriver.hpp"
#include "Static/ModuleInfo.hpp"

RADriver::RADriver(std::shared_ptr<const Config> conf, std::unique_ptr<llvm::Module> mod,
		std::unique_ptr<ModuleInfo> MI, GenMCDriver::Mode mode /* = GenMCDriver::VerificationMode{} */)
	: GenMCDriver(conf, std::move(mod), std::move(MI), mode) {}

void RADriver::visitCalc0_0(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc0_0[lab->getStamp().get()] = NodeStatus::entered;
	calcRes.update(lab->view(0));
	calcRes.updateIdx(lab->getPos());
	if (auto pLab = tc_pred(g, lab); pLab) {
		auto status = visitedCalc0_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_2(pLab, calcRes);
	}
	if (auto pLab = tj_pred(g, lab); pLab) {
		auto status = visitedCalc0_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_2(pLab, calcRes);
	}
	visitedCalc0_0[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc0_1(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc0_1[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = po_imm_pred(g, lab); pLab)if (true && pLab->isAtLeastAcquire()) {
		auto status = visitedCalc0_4[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_4(pLab, calcRes);
	}
	if (auto pLab = po_imm_pred(g, lab); pLab) {
		auto status = visitedCalc0_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_0(pLab, calcRes);
	}
	if (true && lab->isAtLeastAcquire() && llvm::isa<FenceLabel>(lab))if (auto pLab = po_imm_pred(g, lab); pLab) {
		auto status = visitedCalc0_6[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_6(pLab, calcRes);
	}
	if (true && lab->isAtLeastAcquire() && llvm::isa<ThreadJoinLabel>(lab))if (auto pLab = po_imm_pred(g, lab); pLab) {
		auto status = visitedCalc0_6[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_6(pLab, calcRes);
	}
	if (true && lab->isAtLeastAcquire() && llvm::isa<ThreadStartLabel>(lab))if (auto pLab = po_imm_pred(g, lab); pLab) {
		auto status = visitedCalc0_6[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_6(pLab, calcRes);
	}
	visitedCalc0_1[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc0_2(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc0_2[lab->getStamp().get()] = NodeStatus::entered;
	calcRes.update(lab->view(0));
	calcRes.updateIdx(lab->getPos());
	visitedCalc0_2[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc0_3(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc0_3[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = po_imm_pred(g, lab); pLab)if (true && pLab->isAtLeastRelease() && llvm::isa<FenceLabel>(pLab)) {
		auto status = visitedCalc0_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_2(pLab, calcRes);
	}
	if (auto pLab = po_imm_pred(g, lab); pLab)if (true && pLab->isAtLeastRelease() && llvm::isa<ThreadCreateLabel>(pLab)) {
		auto status = visitedCalc0_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_2(pLab, calcRes);
	}
	if (auto pLab = po_imm_pred(g, lab); pLab)if (true && pLab->isAtLeastRelease() && llvm::isa<ThreadFinishLabel>(pLab)) {
		auto status = visitedCalc0_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_2(pLab, calcRes);
	}
	if (auto pLab = po_imm_pred(g, lab); pLab) {
		auto status = visitedCalc0_3[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_3(pLab, calcRes);
	}
	visitedCalc0_3[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc0_4(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc0_4[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = rf_pred(g, lab); pLab)if (true && pLab->isAtLeastRelease()) {
		auto status = visitedCalc0_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_2(pLab, calcRes);
	}
	if (auto pLab = rf_pred(g, lab); pLab) {
		auto status = visitedCalc0_3[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_3(pLab, calcRes);
	}
	if (auto pLab = rf_pred(g, lab); pLab)if (true && llvm::isa<WriteLabel>(pLab) && ((llvm::isa<ReadLabel>(pLab) && g.isRMWLoad(pLab)) || (llvm::isa<WriteLabel>(pLab) && g.isRMWStore(pLab)))) {
		auto status = visitedCalc0_5[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_5(pLab, calcRes);
	}
	visitedCalc0_4[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc0_5(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc0_5[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = po_imm_pred(g, lab); pLab)if (true && llvm::isa<ReadLabel>(pLab) && ((llvm::isa<ReadLabel>(pLab) && g.isRMWLoad(pLab)) || (llvm::isa<WriteLabel>(pLab) && g.isRMWStore(pLab)))) {
		auto status = visitedCalc0_4[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_4(pLab, calcRes);
	}
	visitedCalc0_5[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc0_6(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc0_6[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = rf_pred(g, lab); pLab)if (true && pLab->isAtLeastRelease()) {
		auto status = visitedCalc0_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_2(pLab, calcRes);
	}
	if (auto pLab = rf_pred(g, lab); pLab) {
		auto status = visitedCalc0_3[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_3(pLab, calcRes);
	}
	if (auto pLab = rf_pred(g, lab); pLab)if (true && llvm::isa<WriteLabel>(pLab) && ((llvm::isa<ReadLabel>(pLab) && g.isRMWLoad(pLab)) || (llvm::isa<WriteLabel>(pLab) && g.isRMWStore(pLab)))) {
		auto status = visitedCalc0_5[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_5(pLab, calcRes);
	}
	if (auto pLab = po_imm_pred(g, lab); pLab) {
		auto status = visitedCalc0_6[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc0_6(pLab, calcRes);
	}
	visitedCalc0_6[lab->getStamp().get()] = NodeStatus::left;
}

View RADriver::calculate0(const EventLabel *lab)
{
	auto &g = getGraph();
	View calcRes;

	calcRes.updateIdx(lab->getPos().prev());
	visitedCalc0_0.clear();
	visitedCalc0_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc0_1.clear();
	visitedCalc0_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc0_2.clear();
	visitedCalc0_2.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc0_3.clear();
	visitedCalc0_3.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc0_4.clear();
	visitedCalc0_4.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc0_5.clear();
	visitedCalc0_5.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc0_6.clear();
	visitedCalc0_6.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);

	visitCalc0_1(lab, calcRes);
	return calcRes;
}
void RADriver::visitCalc1_0(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc1_0[lab->getStamp().get()] = NodeStatus::entered;
	calcRes.update(lab->view(1));
	calcRes.updateIdx(lab->getPos());
	if (auto pLab = tc_pred(g, lab); pLab) {
		auto status = visitedCalc1_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc1_2(pLab, calcRes);
	}
	if (auto pLab = tj_pred(g, lab); pLab) {
		auto status = visitedCalc1_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc1_2(pLab, calcRes);
	}
	if (auto pLab = rf_pred(g, lab); pLab) {
		auto status = visitedCalc1_2[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc1_2(pLab, calcRes);
	}
	visitedCalc1_0[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc1_1(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc1_1[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = po_imm_pred(g, lab); pLab) {
		auto status = visitedCalc1_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitCalc1_0(pLab, calcRes);
	}
	visitedCalc1_1[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitCalc1_2(const EventLabel *lab, View &calcRes)
{
	auto &g = getGraph();

	visitedCalc1_2[lab->getStamp().get()] = NodeStatus::entered;
	calcRes.update(lab->view(1));
	calcRes.updateIdx(lab->getPos());
	visitedCalc1_2[lab->getStamp().get()] = NodeStatus::left;
}

View RADriver::calculate1(const EventLabel *lab)
{
	auto &g = getGraph();
	View calcRes;

	calcRes.updateIdx(lab->getPos().prev());
	visitedCalc1_0.clear();
	visitedCalc1_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc1_1.clear();
	visitedCalc1_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedCalc1_2.clear();
	visitedCalc1_2.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);

	visitCalc1_1(lab, calcRes);
	return calcRes;
}
std::vector<VSet<Event>> RADriver::calculateSaved(const EventLabel *lab)
{
	return std::move(saved);
}

std::vector<View> RADriver::calculateViews(const EventLabel *lab)
{
	views.push_back(calculate0(lab));
	views.push_back(calculate1(lab));
	return std::move(views);
}

void RADriver::updateMMViews(EventLabel *lab)
{
	lab->setCalculated(calculateSaved(lab));
	lab->setViews(calculateViews(lab));
	lab->setPrefixView(calculatePrefixView(lab));
}

bool RADriver::isDepTracking() const
{
	return 0;
}

bool RADriver::visitRaceLHS0_0(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS0_0[lab->getStamp().get()] = NodeStatus::entered;
	if (!v.contains(lab->getPos())) {
		racyLab0 = lab;
		return false;
	}
	visitedRaceLHS0_0[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::visitRaceLHS0_1(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS0_1[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = alloc_pred(g, lab); pLab) {
		auto status = visitedRaceLHS0_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS0_0(pLab, v))
			return false;
	}
	visitedRaceLHS0_1[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::checkRace0(const EventLabel *lab) const
{
	auto &g = getGraph();
	auto &v = lab->view(0);

	visitedRaceLHS0_0.clear();
	visitedRaceLHS0_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS0_1.clear();
	visitedRaceLHS0_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	return true
		&& visitRaceLHS0_1(lab, v);
}

void RADriver::visitRaceLHS1_0(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS1_0[lab->getStamp().get()] = NodeStatus::entered;
	lhsAcceptRace1[lab->getStamp().get()] = true;
	visitedRaceLHS1_0[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitRaceLHS1_1(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS1_1[lab->getStamp().get()] = NodeStatus::entered;
	if (true && llvm::isa<FreeLabel>(lab) && !llvm::isa<HpRetireLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<FreeLabel>(pLab) && !llvm::isa<HpRetireLabel>(pLab)) {
		auto status = visitedRaceLHS1_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS1_0(pLab);
	}
	if (true && llvm::isa<FreeLabel>(lab) && !llvm::isa<HpRetireLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<HpRetireLabel>(pLab)) {
		auto status = visitedRaceLHS1_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS1_0(pLab);
	}
	if (true && llvm::isa<HpRetireLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<FreeLabel>(pLab) && !llvm::isa<HpRetireLabel>(pLab)) {
		auto status = visitedRaceLHS1_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS1_0(pLab);
	}
	if (true && llvm::isa<HpRetireLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<HpRetireLabel>(pLab)) {
		auto status = visitedRaceLHS1_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS1_0(pLab);
	}
	visitedRaceLHS1_1[lab->getStamp().get()] = NodeStatus::left;
}

bool RADriver::checkRace1(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS1_0.clear();
	visitedRaceLHS1_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS1_1.clear();
	visitedRaceLHS1_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	lhsAcceptRace1.clear();
	lhsAcceptRace1.resize(g.getMaxStamp().get() + 1, false);
	rhsAcceptRace1.clear();
	rhsAcceptRace1.resize(g.getMaxStamp().get() + 1, false);

	visitRaceLHS1_1(lab);
	for (auto i = 0u; i < lhsAcceptRace1.size(); i++) {
		if (lhsAcceptRace1[i] && !rhsAcceptRace1[i]) {
			racyLab1 = &*std::find_if(label_begin(g), label_end(g), [&](auto &lab){ return lab.getStamp() == i; });
			return false;
		}
	}
	return true;
}

bool RADriver::visitRaceLHS2_0(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS2_0[lab->getStamp().get()] = NodeStatus::entered;
	if (!v.contains(lab->getPos())) {
		racyLab2 = lab;
		return false;
	}
	visitedRaceLHS2_0[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::visitRaceLHS2_1(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS2_1[lab->getStamp().get()] = NodeStatus::entered;
	for (auto &tmp : alloc_succs(g, lab)) if (auto *pLab = &tmp; true) {
		auto status = visitedRaceLHS2_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS2_0(pLab, v))
			return false;
	}
	visitedRaceLHS2_1[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::visitRaceLHS2_2(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS2_2[lab->getStamp().get()] = NodeStatus::entered;
	if (true && llvm::isa<FreeLabel>(lab) && !llvm::isa<HpRetireLabel>(lab))if (auto pLab = free_pred(g, lab); pLab) {
		auto status = visitedRaceLHS2_1[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS2_1(pLab, v))
			return false;
	}
	if (true && llvm::isa<FreeLabel>(lab) && !llvm::isa<HpRetireLabel>(lab))if (auto pLab = free_pred(g, lab); pLab) {
		auto status = visitedRaceLHS2_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS2_0(pLab, v))
			return false;
	}
	visitedRaceLHS2_2[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::checkRace2(const EventLabel *lab) const
{
	auto &g = getGraph();
	auto &v = lab->view(0);

	visitedRaceLHS2_0.clear();
	visitedRaceLHS2_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS2_1.clear();
	visitedRaceLHS2_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS2_2.clear();
	visitedRaceLHS2_2.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	return true
		&& visitRaceLHS2_2(lab, v);
}

void RADriver::visitRaceLHS3_0(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS3_0[lab->getStamp().get()] = NodeStatus::entered;
	lhsAcceptRace3[lab->getStamp().get()] = true;
	visitedRaceLHS3_0[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitRaceLHS3_1(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS3_1[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = free_succ(g, lab); pLab)if (true && llvm::isa<FreeLabel>(pLab) && !llvm::isa<HpRetireLabel>(pLab)) {
		auto status = visitedRaceLHS3_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS3_0(pLab);
	}
	visitedRaceLHS3_1[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitRaceLHS3_2(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS3_2[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = alloc_pred(g, lab); pLab) {
		auto status = visitedRaceLHS3_1[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS3_1(pLab);
	}
	visitedRaceLHS3_2[lab->getStamp().get()] = NodeStatus::left;
}

bool RADriver::checkRace3(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS3_0.clear();
	visitedRaceLHS3_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS3_1.clear();
	visitedRaceLHS3_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS3_2.clear();
	visitedRaceLHS3_2.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	lhsAcceptRace3.clear();
	lhsAcceptRace3.resize(g.getMaxStamp().get() + 1, false);
	rhsAcceptRace3.clear();
	rhsAcceptRace3.resize(g.getMaxStamp().get() + 1, false);

	visitRaceLHS3_2(lab);
	for (auto i = 0u; i < lhsAcceptRace3.size(); i++) {
		if (lhsAcceptRace3[i] && !rhsAcceptRace3[i]) {
			racyLab3 = &*std::find_if(label_begin(g), label_end(g), [&](auto &lab){ return lab.getStamp() == i; });
			return false;
		}
	}
	return true;
}

bool RADriver::visitRaceLHS4_0(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS4_0[lab->getStamp().get()] = NodeStatus::entered;
	if (!v.contains(lab->getPos())) {
		racyLab4 = lab;
		return false;
	}
	visitedRaceLHS4_0[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::visitRaceLHS4_1(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS4_1[lab->getStamp().get()] = NodeStatus::entered;
	for (auto &tmp : alloc_succs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<MemAccessLabel>(pLab) && llvm::dyn_cast<MemAccessLabel>(pLab)->getAddr().isDynamic() && !isHazptrProtected(llvm::dyn_cast<MemAccessLabel>(pLab))) {
		auto status = visitedRaceLHS4_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS4_0(pLab, v))
			return false;
	}
	visitedRaceLHS4_1[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::visitRaceLHS4_2(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS4_2[lab->getStamp().get()] = NodeStatus::entered;
	if (true && llvm::isa<HpRetireLabel>(lab))if (auto pLab = free_pred(g, lab); pLab) {
		auto status = visitedRaceLHS4_1[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS4_1(pLab, v))
			return false;
	}
	if (true && llvm::isa<HpRetireLabel>(lab))if (auto pLab = free_pred(g, lab); pLab)if (true && llvm::isa<MemAccessLabel>(pLab) && llvm::dyn_cast<MemAccessLabel>(pLab)->getAddr().isDynamic() && !isHazptrProtected(llvm::dyn_cast<MemAccessLabel>(pLab))) {
		auto status = visitedRaceLHS4_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS4_0(pLab, v))
			return false;
	}
	visitedRaceLHS4_2[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::checkRace4(const EventLabel *lab) const
{
	auto &g = getGraph();
	auto &v = lab->view(0);

	visitedRaceLHS4_0.clear();
	visitedRaceLHS4_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS4_1.clear();
	visitedRaceLHS4_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS4_2.clear();
	visitedRaceLHS4_2.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	return true
		&& visitRaceLHS4_2(lab, v);
}

void RADriver::visitRaceLHS5_0(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS5_0[lab->getStamp().get()] = NodeStatus::entered;
	lhsAcceptRace5[lab->getStamp().get()] = true;
	visitedRaceLHS5_0[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitRaceLHS5_1(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS5_1[lab->getStamp().get()] = NodeStatus::entered;
	if (auto pLab = free_succ(g, lab); pLab)if (true && llvm::isa<HpRetireLabel>(pLab)) {
		auto status = visitedRaceLHS5_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS5_0(pLab);
	}
	visitedRaceLHS5_1[lab->getStamp().get()] = NodeStatus::left;
}

void RADriver::visitRaceLHS5_2(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS5_2[lab->getStamp().get()] = NodeStatus::entered;
	if (true && llvm::isa<MemAccessLabel>(lab) && llvm::dyn_cast<MemAccessLabel>(lab)->getAddr().isDynamic() && !isHazptrProtected(llvm::dyn_cast<MemAccessLabel>(lab)))if (auto pLab = alloc_pred(g, lab); pLab) {
		auto status = visitedRaceLHS5_1[pLab->getStamp().get()];
		if (status == NodeStatus::unseen)
			visitRaceLHS5_1(pLab);
	}
	visitedRaceLHS5_2[lab->getStamp().get()] = NodeStatus::left;
}

bool RADriver::checkRace5(const EventLabel *lab) const
{
	auto &g = getGraph();

	visitedRaceLHS5_0.clear();
	visitedRaceLHS5_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS5_1.clear();
	visitedRaceLHS5_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS5_2.clear();
	visitedRaceLHS5_2.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	lhsAcceptRace5.clear();
	lhsAcceptRace5.resize(g.getMaxStamp().get() + 1, false);
	rhsAcceptRace5.clear();
	rhsAcceptRace5.resize(g.getMaxStamp().get() + 1, false);

	visitRaceLHS5_2(lab);
	for (auto i = 0u; i < lhsAcceptRace5.size(); i++) {
		if (lhsAcceptRace5[i] && !rhsAcceptRace5[i]) {
			racyLab5 = &*std::find_if(label_begin(g), label_end(g), [&](auto &lab){ return lab.getStamp() == i; });
			return false;
		}
	}
	return true;
}

bool RADriver::visitRaceLHS6_0(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS6_0[lab->getStamp().get()] = NodeStatus::entered;
	if (!v.contains(lab->getPos())) {
		racyLab6 = lab;
		return false;
	}
	visitedRaceLHS6_0[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::visitRaceLHS6_1(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS6_1[lab->getStamp().get()] = NodeStatus::entered;
	if (true && lab->isNotAtomic() && llvm::isa<WriteLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<WriteLabel>(pLab)) {
		auto status = visitedRaceLHS6_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS6_0(pLab, v))
			return false;
	}
	if (true && lab->isNotAtomic() && llvm::isa<WriteLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<ReadLabel>(pLab)) {
		auto status = visitedRaceLHS6_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS6_0(pLab, v))
			return false;
	}
	if (true && lab->isNotAtomic() && llvm::isa<ReadLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<WriteLabel>(pLab)) {
		auto status = visitedRaceLHS6_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS6_0(pLab, v))
			return false;
	}
	if (true && llvm::isa<WriteLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && pLab->isNotAtomic() && llvm::isa<WriteLabel>(pLab)) {
		auto status = visitedRaceLHS6_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS6_0(pLab, v))
			return false;
	}
	if (true && llvm::isa<WriteLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && pLab->isNotAtomic() && llvm::isa<ReadLabel>(pLab)) {
		auto status = visitedRaceLHS6_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS6_0(pLab, v))
			return false;
	}
	if (true && llvm::isa<ReadLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && pLab->isNotAtomic() && llvm::isa<WriteLabel>(pLab)) {
		auto status = visitedRaceLHS6_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS6_0(pLab, v))
			return false;
	}
	visitedRaceLHS6_1[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::checkRace6(const EventLabel *lab) const
{
	auto &g = getGraph();
	auto &v = lab->view(0);

	visitedRaceLHS6_0.clear();
	visitedRaceLHS6_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS6_1.clear();
	visitedRaceLHS6_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	return true
		&& visitRaceLHS6_1(lab, v);
}

bool RADriver::visitRaceLHS7_0(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS7_0[lab->getStamp().get()] = NodeStatus::entered;
	if (!v.contains(lab->getPos())) {
		racyLab7 = lab;
		return false;
	}
	visitedRaceLHS7_0[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::visitRaceLHS7_1(const EventLabel *lab, const View &v) const
{
	auto &g = getGraph();

	visitedRaceLHS7_1[lab->getStamp().get()] = NodeStatus::entered;
	if (true && llvm::isa<WriteLabel>(lab))for (auto &tmp : samelocs(g, lab)) if (auto *pLab = &tmp; true)if (true && llvm::isa<WriteLabel>(pLab)) {
		auto status = visitedRaceLHS7_0[pLab->getStamp().get()];
		if (status == NodeStatus::unseen && !visitRaceLHS7_0(pLab, v))
			return false;
	}
	visitedRaceLHS7_1[lab->getStamp().get()] = NodeStatus::left;
	return true;
}

bool RADriver::checkRace7(const EventLabel *lab) const
{
	auto &g = getGraph();
	auto &v = lab->view(1);

	visitedRaceLHS7_0.clear();
	visitedRaceLHS7_0.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	visitedRaceLHS7_1.clear();
	visitedRaceLHS7_1.resize(g.getMaxStamp().get() + 1, NodeStatus::unseen);
	return true
		&& visitRaceLHS7_1(lab, v);
}

VerificationError RADriver::checkErrors(const EventLabel *lab, const EventLabel *&race) const
{
	if (!checkRace0(lab)) {
		race = racyLab0;
		return VerificationError::VE_AccessNonMalloc;
	}

	if (!checkRace1(lab)) {
		race = racyLab1;
		return VerificationError::VE_DoubleFree;
	}

	if (!checkRace2(lab)) {
		race = racyLab2;
		return VerificationError::VE_AccessFreed;
	}

	if (!checkRace3(lab)) {
		race = racyLab3;
		return VerificationError::VE_AccessFreed;
	}

	if (!checkRace4(lab)) {
		race = racyLab4;
		return VerificationError::VE_AccessFreed;
	}

	if (!checkRace5(lab)) {
		race = racyLab5;
		return VerificationError::VE_AccessFreed;
	}

	if (!checkRace6(lab)) {
		race = racyLab6;
		return VerificationError::VE_RaceNotAtomic;
	}

	return VerificationError::VE_OK;
}

std::vector<VerificationError> RADriver::checkWarnings(const EventLabel *lab, const VSet<VerificationError> &seenWarnings, std::vector<const EventLabel *> &racyLabs) const
{
	std::vector<VerificationError> result;

	if (seenWarnings.count(VerificationError::VE_WWRace) == 0 && !checkRace7(lab)) {
		racyLabs.push_back(racyLab7);
		result.push_back(VerificationError::VE_WWRace);
	}

	return result;
}

bool RADriver::isConsistent(const EventLabel *lab) const
{

	return true;
}

bool RADriver::isRecAcyclic(const EventLabel *lab) const
{
	visitedRecAccepting = 0;
	return true;
}

bool RADriver::isRecoveryValid(const EventLabel *lab) const
{
	return isRecAcyclic(lab);
}

View RADriver::calcPPoRfBefore(const EventLabel *lab) const
{
	auto &g = getGraph();
	View pporf;
	pporf.updateIdx(lab->getPos());

	auto *pLab = g.getPreviousLabel(lab);
	if (!pLab)
		return pporf;
	pporf.update(pLab->getPrefixView());
	if (auto *rLab = llvm::dyn_cast<ReadLabel>(pLab))
		std::ranges::for_each(rLab->getRfs(), [&](const auto &p){
			pporf.update(p.first->getPrefixView());
		});
	if (auto *tsLab = llvm::dyn_cast<ThreadStartLabel>(pLab))
		pporf.update(g.getEventLabel(tsLab->getParentCreate())->getPrefixView());
	if (auto *tjLab = llvm::dyn_cast<ThreadJoinLabel>(pLab))
		pporf.update(g.getLastThreadLabel(tjLab->getChildId())->getPrefixView());
	return pporf;
}
std::unique_ptr<VectorClock> RADriver::calculatePrefixView(const EventLabel *lab) const
{
	return std::make_unique<View>(calcPPoRfBefore(lab));
}

const View &RADriver::getHbView(const EventLabel *lab) const
{
	return lab->view(0);
}


bool RADriver::isWriteRfBefore(Event a, Event b, const AAccess &access) const
{
	auto &g = getGraph();
	auto &before = g.getEventLabel(b)->view(0);
	if (before.contains(a))
		return true;

	const EventLabel *lab = g.getEventLabel(a);

	BUG_ON(!llvm::isa<WriteLabel>(lab));
	auto *wLab = static_cast<const WriteLabel *>(lab);
	for (auto &rLab : wLab->readers())
		if (rLab.overlaps(access) && before.contains(rLab.getPos()))
			return true;
	return false;
}

bool RADriver::isHbOptRfBefore(const Event e, const Event write, const AAccess &access) const
{
	auto &g = getGraph();
	const EventLabel *lab = g.getEventLabel(write);

	BUG_ON(!llvm::isa<WriteLabel>(lab));
	auto *sLab = static_cast<const WriteLabel *>(lab);
	if (sLab->view(0).contains(e))
		return true;

	for (auto &rLab : sLab->readers()) {
		// TODO: Check again
		if (rLab.overlaps(access) && rLab.view(0).contains(e))
			return true;
	}
	return false;
}

ExecutionGraph::co_iterator RADriver::splitLocMOBefore(const MemAccessLabel *mLab)
{
	auto &g = getGraph();
	const auto &access = mLab->getAccess();
	// TODO: Why did MIXER have mLab->getPos().prev() ?
	auto rit = std::find_if(g.co_rbegin(access), g.co_rend(access),
				[&](auto &lab) { return isWriteRfBefore(lab.getPos(),
					mLab->getPos(), access); });
	/* Convert to forward iterator, but be _really_ careful */
	if (rit == g.co_rend(access))
		return g.co_begin(access);

	/* Get the next that overlaps with mLab */
	return g.co_succ_begin(&*rit, mLab->getAccess());
}

ExecutionGraph::co_iterator RADriver::splitLocMOAfterHb(const ReadLabel *rLab)
{
	auto &g = getGraph();

	const auto &access = rLab->getAccess();
	auto initRfs = g.getInitRfsAtLoc(access);
	if (std::any_of(initRfs.begin(), initRfs.end(), [&rLab, &g](const Event &rf) {
		    return g.getEventLabel(rf)->view(0).contains(rLab->getPos());
	    }))
		return g.co_begin(access);

	auto it = std::find_if(g.co_begin(access), g.co_end(access),
			       [&](auto &lab) { return isHbOptRfBefore(rLab->getPos(),
				       lab.getPos(), access); });
	if (it == g.co_end(access) || it->view(0).contains(rLab->getPos()))
		return it;
	return ++it;
}

ExecutionGraph::co_iterator RADriver::splitLocMOAfter(const WriteLabel *sLab)
{
	auto &g = getGraph();
	const auto &access = sLab->getAccess();
	return std::find_if(g.co_begin(access), g.co_end(access),
			    [&](auto &lab) { return isHbOptRfBefore(sLab->getPos(),
				    lab.getPos(), access); });
}

std::vector<Event> RADriver::getCoherentStores(const ReadLabel *rLab)
{
	auto &g = getGraph();
	std::vector<Event> stores;

	/* Under MIXER, inconcistent choices are discarded during rf-enumeration */
	if (getConf()->mixer) {
		const auto &access = rLab->getAccess();
		auto begIt = g.co_begin(access);
		stores.push_back(Event::getInit());
		auto endIt = g.co_end(access);
		std::transform(begIt, endIt, std::back_inserter(stores),
				[&](auto &lab) { return lab.getPos(); });
		return stores;
	}

	/*
	 * If there are no stores (rf?;hb)-before the current event
	 * then we can read read from all concurrent stores and the
	 * initializer store. Otherwise, we can read from all concurrent
	 * stores and the mo-latest of the (rf?;hb)-before stores.
	 */
	auto begIt = splitLocMOBefore(rLab);
	const auto &access = rLab->getAccess();
	if (begIt == g.co_begin(access))
		stores.push_back(Event::getInit());
	else {
		stores.push_back((--begIt)->getPos());
		++begIt;
	}

	/*
	 * If the model supports out-of-order execution we have to also
	 * account for the possibility the read is hb-before some other
	 * store, or some read that reads from a store.
	 */
	auto endIt = (isDepTracking()) ? splitLocMOAfterHb(rLab) : g.co_end(access);
	std::transform(begIt, endIt, std::back_inserter(stores),
		       [&](auto &lab) { return lab.getPos(); });
	return stores;
}

std::vector<Event>
RADriver::getMOOptRfAfter(const WriteLabel *sLab)
{
	std::vector<Event> after;
	std::vector<const ReadLabel *> rfAfter;

	const auto &g = getGraph();
	std::for_each(g.co_succ_begin(sLab), g.co_succ_end(sLab),
		      [&](auto &wLab){
			      after.push_back(wLab.getPos());
			      std::transform(wLab.readers_begin(), wLab.readers_end(), std::back_inserter(rfAfter),
			      [&](auto &rLab){ return &rLab; });
	});
	std::transform(rfAfter.begin(), rfAfter.end(), std::back_inserter(after), [](auto *rLab){
		return rLab->getPos();
	});
	return after;
}

std::vector<Event>
RADriver::getMOInvOptRfAfter(const WriteLabel *sLab)
{
	auto &g = getGraph();
	std::vector<Event> after;
	std::vector<const ReadLabel *> rfAfter;

	/* First, add (mo;rf?)-before */
	std::for_each(g.co_pred_begin(sLab),
		      g.co_pred_end(sLab), [&](auto &wLab){
			      after.push_back(wLab.getPos());
			      std::transform(wLab.readers_begin(), wLab.readers_end(), std::back_inserter(rfAfter),
			      [&](auto &rLab){ return &rLab; });
	});
	std::transform(rfAfter.begin(), rfAfter.end(), std::back_inserter(after), [](auto *rLab){
		return rLab->getPos();
	});

	/* Then, we add the reader list for the initializer */
	auto initRfs = g.getInitRfsAtLoc(sLab->getAccess());
	after.insert(after.end(), initRfs.begin(), initRfs.end());
	return after;
}

std::vector<Event>
RADriver::getCoherentRevisits(const WriteLabel *sLab, const VectorClock &pporf)
{
	auto &g = getGraph();
	auto ls = g.getRevisitable(sLab, pporf);

	/* If this store is po- and mo-maximal then we are done */
	if (!isDepTracking() && g.isCoMaximal(sLab->getAccess(), sLab->getPos()))
		return ls;

	/* TODO: Under MIXER, we might read only part of the write and therefore
	 * the code below is too conservative. RA-inconsistent options are filtered
	 * during the rf-enumeration. */
	if (getConf()->mixer)
		return ls;

	/* First, we have to exclude (mo;rf?;hb?;sb)-after reads */
	auto optRfs = getMOOptRfAfter(sLab);
	ls.erase(std::remove_if(ls.begin(), ls.end(), [&](Event e)
				{ const View &before = g.getEventLabel(e)->view(0);
				  return std::any_of(optRfs.begin(), optRfs.end(),
					 [&](Event ev)
					 { return before.contains(ev); });
				}), ls.end());

	/* If out-of-order event addition is not supported, then we are done
	 * due to po-maximality */
	if (!isDepTracking())
		return ls;

	/* Otherwise, we also have to exclude hb-before loads */
	ls.erase(std::remove_if(ls.begin(), ls.end(), [&](Event e)
		{ return g.getEventLabel(sLab->getPos())->view(0).contains(e); }),
		ls.end());

	/* ...and also exclude (mo^-1; rf?; (hb^-1)?; sb^-1)-after reads in
	 * the resulting graph */
	auto &before = pporf;
	auto moInvOptRfs = getMOInvOptRfAfter(sLab);
	ls.erase(std::remove_if(ls.begin(), ls.end(), [&](Event e)
				{ auto *eLab = g.getEventLabel(e);
				  auto v = g.getViewFromStamp(eLab->getStamp());
				  v->update(before);
				  return std::any_of(moInvOptRfs.begin(),
						     moInvOptRfs.end(),
						     [&](Event ev)
						     { return v->contains(ev) &&
						       g.getEventLabel(ev)->view(0).contains(e); });
				}),
		 ls.end());

	return ls;
}

llvm::iterator_range<ExecutionGraph::co_iterator>
RADriver::getCoherentPlacings(const WriteLabel *wLab)
{
	auto &g = getGraph();
	const auto &access = wLab->getAccess();

	// TODO: Not used under MIXER

	/* If it is an RMW store, there is only one possible position in MO */
	if (g.isRMWStore(wLab)) {
		if (auto *rLab = llvm::dyn_cast<ReadLabel>(g.getPreviousLabel(wLab))) {
			auto *rfLab = rLab->getRf();
			BUG_ON(!rfLab);
			if (auto *wLab = llvm::dyn_cast<WriteLabel>(rfLab)) {
				auto wIt = g.co_succ_begin(wLab);
				return llvm::iterator_range<ExecutionGraph::co_iterator>(wIt, wIt);
			}
			return llvm::iterator_range<ExecutionGraph::co_iterator>(g.co_begin(access),
										 g.co_begin(access));
		}
		BUG();
	}

	/* Otherwise, we calculate the full range and add the store */
	auto rangeBegin = splitLocMOBefore(wLab);
	auto rangeEnd = (isDepTracking()) ? splitLocMOAfter(wLab) : g.co_end(wLab->getAccess());
	return llvm::iterator_range(rangeBegin, rangeEnd);

}
