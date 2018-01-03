#include "filesys.h"
#include "treemodel.h"
#include "sortfilterproxy.h"
#include "maingui.h"
#include <QApplication>
#include <QtGui>
#include <QWidget>
#include <QTreeView>



int main(int argc, char **argv)
{
	QApplication app(argc, argv);

	path  left("/home/martin/Dropbox/Programming/tools_and_snippets/cpp_snippets/");
	path right("/home/martin/Dropbox/Programming/tools_and_snippets/cpp_snippets_copy/");

	auto difftree = fsdiff::compare(left, right);

	MainGui gui( difftree );
	gui.show();

//	TreeModel model(nullptr);
//	QTreeView view(nullptr);
//	SortFilterProxy sfp;
//
//	sfp.setSourceModel(&model);
//
//	view.setModel(&sfp);
//	view.setMinimumWidth(800);
//	view.setMinimumHeight(800);
//	view.show();
//
//	//sfp.setFilterRole( Qt::DisplayRole );
//	//sfp.setFilterKeyColumn(0);

	return app.exec();

	//filesys::diff_tree(path("/home/martin/Dropbox/Programming/tools_and_snippets/cpp_snippets/"), path("/home/martin/Dropbox/Programming/tools_and_snippets/cpp_snippets_copy/"));
}